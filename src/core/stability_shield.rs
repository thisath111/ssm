// stability_shield.rs
// Dynamic System Stability Guard & Process Immunity Engine.
// Uses runtime OS window enumeration and session discovery instead of hardcoded 3rd party lists.

use std::collections::HashSet;
use sysinfo::System;
use windows::Win32::Foundation::CloseHandle;
use windows::Win32::System::Threading::{
    GetProcessHandleCount, OpenProcess, PROCESS_QUERY_INFORMATION,
};

pub struct StabilityShield;

impl StabilityShield {
    /// Dynamically verifies if a process is immune from throttling, suspension, or aggressive trimming.
    /// Protects:
    /// 1. System core infrastructure (PID <= 4, Self PID)
    /// 2. Windows Kernel executive subsystem base modules
    #[must_use] 
    pub fn is_immune(pid: u32, process_name: &str) -> bool {
        if pid <= 4 || pid == std::process::id() {
            return true;
        }

        // Windows kernel-level core architecture names (OS-mandated base modules)
        let name_lower = process_name.to_lowercase();
        name_lower == "system"
            || name_lower == "smss.exe"
            || name_lower == "csrss.exe"
            || name_lower == "wininit.exe"
            || name_lower == "winlogon.exe"
            || name_lower == "lsass.exe"
            || name_lower == "services.exe"
            || name_lower == "dwm.exe"
            || name_lower == "fontdrvhost.exe"
            || name_lower == "audiodg.exe"
            || name_lower == "explorer.exe"
            || name_lower.contains("ssm")
    }

    /// Dynamically checks if a process owns an active, hidden, tray, or hook window in the user session.
    #[must_use] 
    pub fn is_interactive_window_owner(pid: u32, window_pids: &HashSet<u32>) -> bool {
        window_pids.contains(&pid)
    }

    /// Queries open Win32 handle count for given PID.
    #[must_use] 
    pub fn get_handle_count(pid: u32) -> u32 {
        if pid <= 4 {
            return 0;
        }
        unsafe {
            if let Ok(handle) = OpenProcess(PROCESS_QUERY_INFORMATION, false, pid) {
                let mut count: u32 = 0;
                let res = GetProcessHandleCount(handle, &raw mut count);
                let _ = CloseHandle(handle);
                if res.is_ok() {
                    return count;
                }
            }
        }
        0
    }

    /// Scans for leaking processes that exceed handle warning threshold (10,000 handles) to prevent kernel pool exhaustion.
    #[must_use] 
    pub fn audit_handle_leaks(sys: &System) -> Vec<(u32, String, u32)> {
        let mut leaking = Vec::new();
        for (pid, process) in sys.processes() {
            let p_u32 = pid.as_u32();
            let name = process.name().to_string_lossy().to_string();
            if Self::is_immune(p_u32, &name) {
                continue;
            }

            let handle_count = Self::get_handle_count(p_u32);
            if handle_count > 10_000 {
                leaking.push((p_u32, name, handle_count));
            }
        }
        leaking
    }
}
