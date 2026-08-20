// priority_balancer.rs
// Ensures the foreground app ALWAYS gets priority CPU time.
// Lowers background CPU hog priorities to prevent UI starvation freezes.

use windows::Win32::System::Threading::{
    OpenProcess, SetPriorityClass, GetPriorityClass,
    PROCESS_SET_INFORMATION, PROCESS_QUERY_INFORMATION,
    ABOVE_NORMAL_PRIORITY_CLASS, BELOW_NORMAL_PRIORITY_CLASS,
    NORMAL_PRIORITY_CLASS, IDLE_PRIORITY_CLASS,
};
use windows::Win32::Foundation::CloseHandle;
use sysinfo::System;
use std::collections::HashMap;
use log::info;

pub struct PriorityBalancer {
    /// Track which PIDs we've modified so we can restore them
    modified_pids: HashMap<u32, u32>, // PID -> original priority class
    current_foreground_pid: u32,
}

impl PriorityBalancer {
    pub fn new() -> Self {
        Self {
            modified_pids: HashMap::new(),
            current_foreground_pid: 0,
        }
    }

    /// Boost foreground, lower background CPU hogs
    pub fn balance(&mut self, sys: &System, foreground_pid: u32) {
        let fg_changed = foreground_pid != self.current_foreground_pid && foreground_pid != 0;

        if fg_changed {
            // Restore previous foreground to normal
            if self.current_foreground_pid != 0 {
                self.restore_priority(self.current_foreground_pid);
            }

            // Boost new foreground
            self.boost_foreground(foreground_pid);
            self.current_foreground_pid = foreground_pid;
        }

        // Lower background CPU hogs
        self.lower_background_hogs(sys, foreground_pid);
    }

    fn boost_foreground(&mut self, pid: u32) {
        if pid == 0 || pid == 4 {
            return;
        }

        unsafe {
            let access = PROCESS_SET_INFORMATION | PROCESS_QUERY_INFORMATION;
            if let Ok(handle) = OpenProcess(access, false, pid) {
                let original = GetPriorityClass(handle);
                if original != 0 && original != ABOVE_NORMAL_PRIORITY_CLASS.0 {
                    // Save original and boost
                    self.modified_pids.insert(pid, original);
                    let _ = SetPriorityClass(handle, ABOVE_NORMAL_PRIORITY_CLASS);
                }
                let _ = CloseHandle(handle);
            }
        }
    }

    fn lower_background_hogs(&mut self, sys: &System, foreground_pid: u32) {
        let self_pid = std::process::id();
        
        let critical_names = [
            "explorer", "csrss", "winlogon", "lsass", "svchost", "dwm",
            "smss", "audiodg", "system", "registry", "wininit", "services",
        ];

        for (pid, process) in sys.processes() {
            let pid_u32 = pid.as_u32();

            // Skip foreground, self, and system processes
            if pid_u32 == foreground_pid || pid_u32 == self_pid || pid_u32 <= 4 {
                continue;
            }

            let name = process.name().to_string_lossy().to_lowercase();
            if critical_names.iter().any(|&c| name.contains(c)) {
                continue;
            }

            let cpu = process.cpu_usage();

            // Only lower priority of processes using significant CPU in background
            if cpu > 15.0 {
                unsafe {
                    let access = PROCESS_SET_INFORMATION | PROCESS_QUERY_INFORMATION;
                    if let Ok(handle) = OpenProcess(access, false, pid_u32) {
                        let current = GetPriorityClass(handle);
                        
                        // Only lower if currently Normal or higher
                        if current == NORMAL_PRIORITY_CLASS.0 || current == ABOVE_NORMAL_PRIORITY_CLASS.0 {
                            if !self.modified_pids.contains_key(&pid_u32) {
                                self.modified_pids.insert(pid_u32, current);
                            }
                            let _ = SetPriorityClass(handle, BELOW_NORMAL_PRIORITY_CLASS);
                        }
                        let _ = CloseHandle(handle);
                    }
                }
            } else if cpu < 5.0 {
                // Restore priority if process is no longer hogging CPU
                self.restore_priority(pid_u32);
            }
        }
    }

    fn restore_priority(&mut self, pid: u32) {
        if let Some(original) = self.modified_pids.remove(&pid) {
            unsafe {
                let access = PROCESS_SET_INFORMATION;
                if let Ok(handle) = OpenProcess(access, false, pid) {
                    let _ = SetPriorityClass(
                        handle,
                        windows::Win32::System::Threading::PROCESS_CREATION_FLAGS(original),
                    );
                    let _ = CloseHandle(handle);
                }
            }
        }
    }

    /// Restore all modified priorities on shutdown
    pub fn restore_all(&mut self) {
        let pids: Vec<u32> = self.modified_pids.keys().copied().collect();
        for pid in pids {
            self.restore_priority(pid);
        }
    }
}

impl Drop for PriorityBalancer {
    fn drop(&mut self) {
        self.restore_all();
    }
}
