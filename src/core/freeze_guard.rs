/// FreezeGuard: Dedicated high-priority watchdog thread that monitors system
/// responsiveness independently of the main engine tick loop.
///
/// The main engine tick runs on a 1-second interval. If the system is under extreme
/// load (100% CPU, 99% RAM), even the main tick can stall. FreezeGuard runs on its
/// own thread at ABOVE_NORMAL priority with a lightweight heartbeat — if the system
/// stops responding, it aggressively kills the worst offending processes to restore
/// responsiveness before a full hard-lock occurs.

use std::sync::atomic::{AtomicU64, AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use windows::Win32::System::Threading::{
    OpenProcess, SetPriorityClass, GetCurrentThread, SetThreadPriority,
    PROCESS_TERMINATE, PROCESS_SET_INFORMATION, IDLE_PRIORITY_CLASS,
    THREAD_PRIORITY_ABOVE_NORMAL,
};
use windows::Win32::Foundation::CloseHandle;
use log::{warn, info};
use sysinfo::System;
use crate::core::stability_shield::StabilityShield;

/// Shared heartbeat counter — the main engine increments this every tick.
/// If FreezeGuard detects it hasn't changed for N seconds, the main loop is stalled.
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
    /// Spawn the FreezeGuard watchdog on a dedicated thread.
    /// It monitors two things independently:
    /// 1. System-wide responsiveness (RAM + CPU thresholds)
    /// 2. Main engine heartbeat (is the tick loop itself frozen?)
    pub fn spawn(heartbeat: Arc<FreezeGuardHeartbeat>) -> std::thread::JoinHandle<()> {
        std::thread::Builder::new()
            .name("ssm-freeze-guard".into())
            .spawn(move || {
                // Elevate this thread to above-normal priority so it runs even under extreme load
                unsafe {
                    let _ = SetThreadPriority(GetCurrentThread(), THREAD_PRIORITY_ABOVE_NORMAL);
                }

                let mut consecutive_critical = 0u32;
                let mut last_seen_tick: u64 = 0;
                let mut stall_count = 0u32;

                loop {
                    if heartbeat.shutdown.load(Ordering::Relaxed) {
                        break;
                    }

                    std::thread::sleep(Duration::from_secs(2));

                    // --- Check 1: Is the main engine tick loop stalled? ---
                    let current_tick = heartbeat.tick.load(Ordering::Relaxed);
                    if current_tick == last_seen_tick && current_tick > 0 {
                        stall_count += 1;
                        if stall_count >= 3 {
                            // Main loop hasn't ticked for 6+ seconds — system is severely stressed
                            warn!("[FreezeGuard] Main engine stalled for {}s — executing emergency intervention", stall_count * 2);
                            Self::emergency_kill_top_hogs();
                            stall_count = 0;
                        }
                    } else {
                        stall_count = 0;
                    }
                    last_seen_tick = current_tick;

                    // --- Check 2: System-wide resource crisis ---
                    let (ram_percent, available_mb) = Self::quick_ram_check();
                    let cpu_percent = Self::quick_cpu_check();

                    let is_crisis = (ram_percent > 95.0 && available_mb < 400)
                        || (cpu_percent > 97.0 && ram_percent > 90.0);

                    if is_crisis {
                        consecutive_critical += 1;
                        if consecutive_critical >= 2 {
                            // 4+ seconds of sustained crisis — intervene
                            warn!(
                                "[FreezeGuard] System crisis detected (CPU: {:.0}%, RAM: {:.0}%, Free: {} MB) — forcing recovery",
                                cpu_percent, ram_percent, available_mb
                            );
                            Self::emergency_kill_top_hogs();
                            Self::emergency_purge_standby();
                            consecutive_critical = 0;
                        }
                    } else {
                        consecutive_critical = 0;
                    }
                }
            })
            .expect("Failed to spawn FreezeGuard thread")
    }

    /// Lightweight RAM check using GlobalMemoryStatusEx (takes <1us, never blocks)
    fn quick_ram_check() -> (f32, u64) {
        use windows::Win32::System::SystemInformation::{GlobalMemoryStatusEx, MEMORYSTATUSEX};
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

    /// Quick CPU estimate from sysinfo (lightweight refresh)
    fn quick_cpu_check() -> f32 {
        let mut sys = System::new();
        sys.refresh_cpu_usage();
        std::thread::sleep(Duration::from_millis(200));
        sys.refresh_cpu_usage();
        sys.global_cpu_usage()
    }

    /// Nuclear option: find the top 3 non-system processes consuming the most RAM
    /// and forcefully terminate them to prevent a full system freeze.
    fn emergency_kill_top_hogs() {
        let mut sys = System::new_all();
        sys.refresh_all();

        let self_pid = std::process::id();
        let foreground_pid = crate::utils::win32::get_foreground_hwnd()
            .map(|h| crate::utils::win32::get_process_id_from_hwnd(h))
            .unwrap_or(0);

        // Collect non-immune processes sorted by memory usage (descending)
        let mut candidates: Vec<(u32, u64, String)> = Vec::new();
        for (pid, process) in sys.processes() {
            let p = pid.as_u32();
            let name = process.name().to_string_lossy().to_lowercase();

            if p <= 4 || p == self_pid || p == foreground_pid || StabilityShield::is_immune(p, &name) {
                continue;
            }

            candidates.push((p, process.memory(), name));
        }

        candidates.sort_by(|a, b| b.1.cmp(&a.1));

        // First pass: throttle top 5 hogs to IDLE priority
        for (pid, mem, name) in candidates.iter().take(5) {
            warn!("[FreezeGuard] Throttling PID {} ({}) — {} MB", pid, name, mem / (1024 * 1024));
            unsafe {
                if let Ok(handle) = OpenProcess(PROCESS_SET_INFORMATION, false, *pid) {
                    let _ = SetPriorityClass(handle, IDLE_PRIORITY_CLASS);
                    let _ = CloseHandle(handle);
                }
            }
        }

        // Second pass: if available RAM is still critically low, terminate the worst offender
        let (_, avail) = Self::quick_ram_check();
        if avail < 200 {
            if let Some((pid, mem, name)) = candidates.first() {
                warn!("[FreezeGuard] CRITICAL: Terminating PID {} ({}) — {} MB to prevent system crash",
                    pid, name, mem / (1024 * 1024));
                unsafe {
                    if let Ok(handle) = OpenProcess(PROCESS_TERMINATE, false, *pid) {
                        let _ = windows::Win32::System::Threading::TerminateProcess(handle, 1);
                        let _ = CloseHandle(handle);
                    }
                }
            }
        }
    }

    /// Force-purge standby memory during crisis
    fn emergency_purge_standby() {
        info!("[FreezeGuard] Emergency standby memory purge");
        let ram_mgr = crate::core::ram::RamManager::new();
        ram_mgr.purge_standby_memory();
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
        // Should return valid values on any Windows machine
        assert!(percent >= 0.0 && percent <= 100.0);
        assert!(avail > 0);
    }
}
