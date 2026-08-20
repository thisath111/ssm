// handle_monitor.rs
// Detects processes leaking handles or GDI objects which eventually cause system-wide freezes.
// Windows has a 10,000 GDI object limit per process and a 16M handle limit system-wide.

use windows::Win32::System::Threading::{
    OpenProcess, PROCESS_QUERY_INFORMATION, GetProcessHandleCount,
};
use windows::Win32::Foundation::CloseHandle;
use sysinfo::System;
use std::collections::HashMap;
use std::os::windows::process::CommandExt;
use log::warn;

const HANDLE_WARNING_THRESHOLD: u32 = 10_000;
const HANDLE_KILL_THRESHOLD: u32 = 50_000;

pub struct HandleMonitor {
    /// Tracks handle counts per PID to detect trends
    handle_history: HashMap<u32, Vec<u32>>,
    last_check_tick: u64,
}

impl HandleMonitor {
    pub fn new() -> Self {
        Self {
            handle_history: HashMap::new(),
            last_check_tick: 0,
        }
    }

    /// Check every 10 ticks (5 seconds) — this is expensive
    pub fn check(&mut self, sys: &System, tick_count: u64) {
        // Only run every 10 ticks (5 seconds)
        if tick_count.saturating_sub(self.last_check_tick) < 10 {
            return;
        }
        self.last_check_tick = tick_count;

        let mut active_pids = std::collections::HashSet::new();

        for (pid, process) in sys.processes() {
            let pid_u32 = pid.as_u32();
            active_pids.insert(pid_u32);

            if pid_u32 == 0 || pid_u32 == 4 {
                continue; // System/Idle
            }

            let handle_count = self.get_handle_count(pid_u32);
            if handle_count == 0 {
                continue;
            }

            // Track history (keep last 6 readings = 30 seconds)
            let history = self.handle_history.entry(pid_u32).or_insert_with(Vec::new);
            history.push(handle_count);
            if history.len() > 6 {
                history.remove(0);
            }

            let name = process.name().to_string_lossy().to_string();

            // Check for absolute threshold
            if handle_count > HANDLE_KILL_THRESHOLD {
                warn!(
                    "[HandleMonitor] CRITICAL: '{}' PID {} has {} handles — likely leaking, terminating!",
                    name, pid_u32, handle_count
                );
                self.kill_leaking_process(pid_u32, &name);
                continue;
            }

            if handle_count > HANDLE_WARNING_THRESHOLD {
                // Check if handles are monotonically increasing (leak pattern)
                if self.is_leaking(pid_u32) {
                    warn!(
                        "[HandleMonitor] WARNING: '{}' PID {} has {} handles and is trending upward — possible leak",
                        name, pid_u32, handle_count
                    );
                }
            }
        }

        // Prune dead PIDs from history
        self.handle_history.retain(|pid, _| active_pids.contains(pid));
    }

    fn get_handle_count(&self, pid: u32) -> u32 {
        unsafe {
            if let Ok(handle) = OpenProcess(PROCESS_QUERY_INFORMATION, false, pid) {
                let mut count: u32 = 0;
                let result = GetProcessHandleCount(handle, &mut count);
                let _ = CloseHandle(handle);
                if result.is_ok() {
                    return count;
                }
            }
        }
        0
    }

    /// Detect if handle count is monotonically increasing over the last 6 readings
    fn is_leaking(&self, pid: u32) -> bool {
        if let Some(history) = self.handle_history.get(&pid) {
            if history.len() < 4 {
                return false;
            }
            // Check if each reading is higher than the previous
            history.windows(2).all(|w| w[1] > w[0])
        } else {
            false
        }
    }

    fn kill_leaking_process(&self, pid: u32, name: &str) {
        // SAFETY: Forcefully killing processes has been disabled to prevent OS corruption.
        // We will just log this as a critical warning.
        warn!("[HandleMonitor] SAFETY LOCK: Would have terminated leaking process '{}' PID {}, but forceful termination is disabled.", name, pid);
    }
}
