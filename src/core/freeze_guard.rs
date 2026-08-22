/// FreezeGuard: Ultra-high-priority watchdog thread.
/// Sub-second freeze detection with instant NtSuspendProcess-based recovery.

use std::sync::atomic::{AtomicU64, AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use windows::Win32::System::Threading::{
    OpenProcess, GetCurrentThread, SetThreadPriority,
    PROCESS_SET_INFORMATION, PROCESS_QUERY_INFORMATION, PROCESS_SUSPEND_RESUME,
    PROCESS_SET_QUOTA, THREAD_PRIORITY_HIGHEST,
};

extern "system" {
    fn GetSystemTimes(
        lpIdleTime: *mut FILETIME,
        lpKernelTime: *mut FILETIME,
        lpUserTime: *mut FILETIME,
    ) -> i32;
}
use windows::Win32::System::SystemInformation::{GlobalMemoryStatusEx, MEMORYSTATUSEX};
use windows::Win32::System::Memory::{SetProcessWorkingSetSizeEx, SETPROCESSWORKINGSETSIZEEX_FLAGS};
use windows::Win32::Foundation::{CloseHandle, FILETIME};
use windows::Win32::System::ProcessStatus::{EnumProcesses, GetProcessMemoryInfo, PROCESS_MEMORY_COUNTERS};
use log::{warn, info};
use crate::core::stability_shield::StabilityShield;

pub struct FreezeGuardHeartbeat {
    pub tick: AtomicU64,
    pub shutdown: AtomicBool,
}

impl FreezeGuardHeartbeat {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            tick: AtomicU64::new(0),
            shutdown: AtomicBool::new(false),
        })
    }
}

pub struct FreezeGuard;

impl FreezeGuard {
    pub fn spawn(heartbeat: Arc<FreezeGuardHeartbeat>) -> std::thread::JoinHandle<()> {
        std::thread::Builder::new()
            .name("ssm-freeze-guard".into())
            .spawn(move || {
                // Highest thread priority so kernel schedules us even at 100% CPU
                unsafe {
                    let _ = SetThreadPriority(GetCurrentThread(), THREAD_PRIORITY_HIGHEST);
                }

                let mut consecutive_critical = 0u32;
                let mut last_seen_tick: u64 = 0;
                let mut stall_count = 0u32;
                let mut prev_idle: u64 = 0;
                let mut prev_total: u64 = 0;

                loop {
                    if heartbeat.shutdown.load(Ordering::Relaxed) {
                        break;
                    }

                    // 500ms check interval for sub-second detection
                    std::thread::sleep(Duration::from_millis(500));

                    // Check 1: Main engine stall (2 consecutive = 1 second)
                    let current_tick = heartbeat.tick.load(Ordering::Relaxed);
                    if current_tick == last_seen_tick && current_tick > 0 {
                        stall_count += 1;
                        if stall_count >= 2 {
                            warn!("[FreezeGuard] Engine stalled for ~{}ms — instant recovery", stall_count * 500);
                            Self::instant_recovery();
                            stall_count = 0;
                        }
                    } else {
                        stall_count = 0;
                    }
                    last_seen_tick = current_tick;

                    // Check 2: System resource crisis (zero-allocation)
                    let (ram_percent, available_mb) = Self::quick_ram_check();
                    let cpu_percent = Self::zero_alloc_cpu_check(&mut prev_idle, &mut prev_total);

                    let is_crisis = (ram_percent > 95.0 && available_mb < 400)
                        || (cpu_percent > 97.0 && ram_percent > 90.0)
                        || available_mb < 200;

                    if is_crisis {
                        consecutive_critical += 1;
                        if consecutive_critical >= 2 {
                            warn!(
                                "[FreezeGuard] CRISIS (CPU: {:.0}%, RAM: {:.0}%, Free: {} MB) — instant recovery",
                                cpu_percent, ram_percent, available_mb
                            );
                            Self::instant_recovery();
                            consecutive_critical = 0;
                        }
                    } else {
                        consecutive_critical = 0;
                    }
                }
            })
            .expect("Failed to spawn FreezeGuard thread")
    }

    /// Zero-allocation CPU check using GetSystemTimes() — takes <1μs instead of 200ms+.
    fn zero_alloc_cpu_check(prev_idle: &mut u64, prev_total: &mut u64) -> f32 {
        let mut idle = FILETIME::default();
        let mut kernel = FILETIME::default();
        let mut user = FILETIME::default();

        let ok = unsafe {
            GetSystemTimes(
                &mut idle,
                &mut kernel,
                &mut user,
            )
        };

        if ok == 0 {
            return 0.0;
        }

        let idle_ticks = ((idle.dwHighDateTime as u64) << 32) | idle.dwLowDateTime as u64;
        let kernel_ticks = ((kernel.dwHighDateTime as u64) << 32) | kernel.dwLowDateTime as u64;
        let user_ticks = ((user.dwHighDateTime as u64) << 32) | user.dwLowDateTime as u64;
        let total_ticks = kernel_ticks + user_ticks;

        if *prev_total == 0 {
            *prev_idle = idle_ticks;
            *prev_total = total_ticks;
            return 0.0;
        }

        let delta_idle = idle_ticks.saturating_sub(*prev_idle);
        let delta_total = total_ticks.saturating_sub(*prev_total);

        *prev_idle = idle_ticks;
        *prev_total = total_ticks;

        if delta_total == 0 {
            return 0.0;
        }

        ((delta_total - delta_idle) as f32 / delta_total as f32) * 100.0
    }

    pub fn quick_ram_check() -> (f32, u64) {
        let mut mem = MEMORYSTATUSEX {
            dwLength: std::mem::size_of::<MEMORYSTATUSEX>() as u32,
            ..Default::default()
        };
        if unsafe { GlobalMemoryStatusEx(&mut mem) }.is_ok() {
            let total = mem.ullTotalPhys / (1024 * 1024);
            let avail = mem.ullAvailPhys / (1024 * 1024);
            let used = total.saturating_sub(avail);
            let percent = if total > 0 { (used as f32 / total as f32) * 100.0 } else { 0.0 };
            (percent, avail)
        } else {
            (0.0, u64::MAX)
        }
    }

    /// Instant recovery: Suspend top hogs + empty working sets + purge standby.
    /// Uses NtSuspendProcess for instant freeze (<1ms per process) instead of slow taskkill.
    fn instant_recovery() {
        // Step 1: Enumerate processes with zero-allocation pre-sized buffer
        let mut pids = [0u32; 2048];
        let mut bytes_returned: u32 = 0;

        let ok = unsafe {
            EnumProcesses(
                pids.as_mut_ptr(),
                (pids.len() * std::mem::size_of::<u32>()) as u32,
                &mut bytes_returned,
            )
        };

        if ok.is_err() {
            return;
        }

        let count = bytes_returned as usize / std::mem::size_of::<u32>();
        let self_pid = std::process::id();
        let foreground_pid = crate::utils::win32::get_foreground_hwnd()
            .map(|h| crate::utils::win32::get_process_id_from_hwnd(h))
            .unwrap_or(0);

        // Step 2: Collect memory usage for each process (zero-allocation: stack array)
        let mut candidates: [(u32, u64); 64] = [(0, 0); 64];
        let mut candidate_count = 0usize;

        for i in 0..count {
            let pid = pids[i];
            if pid <= 4 || pid == self_pid || pid == foreground_pid {
                continue;
            }

            // Get process name to check immunity
            let name = Self::get_process_name_fast(pid);
            if StabilityShield::is_immune(pid, &name) {
                continue;
            }

            // Get memory usage
            let mem = Self::get_process_memory(pid);
            if mem > 0 && candidate_count < 64 {
                candidates[candidate_count] = (pid, mem);
                candidate_count += 1;
            }
        }

        // Step 3: Sort by memory (descending) — simple insertion sort on stack array
        let slice = &mut candidates[..candidate_count];
        for i in 1..slice.len() {
            let mut j = i;
            while j > 0 && slice[j].1 > slice[j - 1].1 {
                slice.swap(j, j - 1);
                j -= 1;
            }
        }

        // Step 4: Instantly suspend top 5 memory hogs via NtSuspendProcess
        let suspend_count = candidate_count.min(5);
        for i in 0..suspend_count {
            let (pid, mem) = slice[i];
            warn!("[FreezeGuard] SUSPENDING PID {} — {} MB", pid, mem / (1024 * 1024));
            Self::suspend_and_empty(pid);
        }

        // Step 5: Empty working sets of next 20 processes
        let trim_end = candidate_count.min(25);
        for i in suspend_count..trim_end {
            let (pid, _) = slice[i];
            Self::empty_working_set(pid);
        }

        // Step 6: Purge standby memory
        info!("[FreezeGuard] Emergency standby purge");
        crate::utils::nt_api::purge_standby_list();

        // Step 7: Auto-resume suspended processes after 5 seconds
        let suspended: Vec<u32> = slice[..suspend_count].iter().map(|(pid, _)| *pid).collect();
        std::thread::spawn(move || {
            std::thread::sleep(Duration::from_secs(5));
            for pid in suspended {
                Self::resume_process(pid);
                info!("[FreezeGuard] Resumed PID {}", pid);
            }
        });
    }

    fn get_process_name_fast(pid: u32) -> String {
        use windows::Win32::System::Threading::PROCESS_QUERY_LIMITED_INFORMATION;
        use windows::Win32::System::Threading::QueryFullProcessImageNameW;
        use windows::Win32::System::Threading::PROCESS_NAME_FORMAT;

        unsafe {
            if let Ok(handle) = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid) {
                let mut buf = [0u16; 260];
                let mut len = buf.len() as u32;
                if QueryFullProcessImageNameW(handle, PROCESS_NAME_FORMAT(0), windows::core::PWSTR(buf.as_mut_ptr()), &mut len).is_ok() {
                    let _ = CloseHandle(handle);
                    let path = String::from_utf16_lossy(&buf[..len as usize]);
                    return path.rsplit('\\').next().unwrap_or("").to_lowercase();
                }
                let _ = CloseHandle(handle);
            }
        }
        String::new()
    }

    fn get_process_memory(pid: u32) -> u64 {
        unsafe {
            if let Ok(handle) = OpenProcess(PROCESS_QUERY_INFORMATION, false, pid) {
                let mut counters = PROCESS_MEMORY_COUNTERS::default();
                counters.cb = std::mem::size_of::<PROCESS_MEMORY_COUNTERS>() as u32;
                if GetProcessMemoryInfo(handle, &mut counters, counters.cb).is_ok() {
                    let _ = CloseHandle(handle);
                    return counters.WorkingSetSize as u64;
                }
                let _ = CloseHandle(handle);
            }
        }
        0
    }

    fn suspend_and_empty(pid: u32) {
        unsafe {
            if let Ok(handle) = OpenProcess(
                PROCESS_SUSPEND_RESUME | PROCESS_SET_QUOTA | PROCESS_QUERY_INFORMATION | PROCESS_SET_INFORMATION,
                false, pid
            ) {
                // Instant freeze via NtSuspendProcess
                ntapi::ntpsapi::NtSuspendProcess(handle.0 as _);
                // Empty working set to free RAM immediately
                let _ = SetProcessWorkingSetSizeEx(
                    handle,
                    usize::MAX, usize::MAX,
                    SETPROCESSWORKINGSETSIZEEX_FLAGS(0x00000002),
                );
                let _ = CloseHandle(handle);
            }
        }
    }

    fn empty_working_set(pid: u32) {
        unsafe {
            if let Ok(handle) = OpenProcess(PROCESS_SET_QUOTA | PROCESS_QUERY_INFORMATION, false, pid) {
                let _ = SetProcessWorkingSetSizeEx(
                    handle,
                    usize::MAX, usize::MAX,
                    SETPROCESSWORKINGSETSIZEEX_FLAGS(0x00000002),
                );
                let _ = CloseHandle(handle);
            }
        }
    }

    fn resume_process(pid: u32) {
        unsafe {
            if let Ok(handle) = OpenProcess(PROCESS_SUSPEND_RESUME, false, pid) {
                ntapi::ntpsapi::NtResumeProcess(handle.0 as _);
                let _ = CloseHandle(handle);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heartbeat_creation() {
        let hb = FreezeGuardHeartbeat::new();
        assert_eq!(hb.tick.load(Ordering::Relaxed), 0);
        assert!(!hb.shutdown.load(Ordering::Relaxed));

        hb.tick.store(42, Ordering::Relaxed);
        assert_eq!(hb.tick.load(Ordering::Relaxed), 42);
    }

    #[test]
    fn test_quick_ram_check() {
        let (percent, avail) = FreezeGuard::quick_ram_check();
        assert!(percent >= 0.0 && percent <= 100.0);
        assert!(avail > 0);
    }

    #[test]
    fn test_zero_alloc_cpu_check() {
        let mut prev_idle = 0u64;
        let mut prev_total = 0u64;
        let cpu = FreezeGuard::zero_alloc_cpu_check(&mut prev_idle, &mut prev_total);
        assert!(cpu >= 0.0 && cpu <= 100.0);
        // Second call should produce a real value
        std::thread::sleep(std::time::Duration::from_millis(50));
        let cpu2 = FreezeGuard::zero_alloc_cpu_check(&mut prev_idle, &mut prev_total);
        assert!(cpu2 >= 0.0 && cpu2 <= 100.0);
    }
}
