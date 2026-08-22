use sysinfo::System;
use windows::Win32::System::Threading::{OpenProcess, PROCESS_QUERY_INFORMATION, GetProcessHandleCount};
use windows::Win32::Foundation::CloseHandle;

const CRITICAL_IMMUNITY_LIST: [&str; 19] = [
    "system", "registry", "smss", "csrss", "wininit", "winlogon",
    "lsass", "services", "svchost", "dwm", "fontdrvhost", "audiodg",
    "ctfmon", "sihost", "textinputhost", "startmenuexperiencehost",
    "shellexperiencehost", "explorer", "ssm",
];

pub struct StabilityShield;

impl StabilityShield {
    /// Verifies absolute immunity for Windows core system infrastructure.
    pub fn is_immune(pid: u32, process_name: &str) -> bool {
        if pid <= 4 || pid == std::process::id() {
            return true;
        }
        let name_lower = process_name.to_lowercase();
        CRITICAL_IMMUNITY_LIST.iter().any(|&c| name_lower.contains(c))
    }

    /// Queries open Win32 handle count for given PID.
    pub fn get_handle_count(pid: u32) -> u32 {
        if pid <= 4 {
            return 0;
        }
        unsafe {
            if let Ok(handle) = OpenProcess(PROCESS_QUERY_INFORMATION, false, pid) {
                let mut count: u32 = 0;
                let res = GetProcessHandleCount(handle, &mut count);
                let _ = CloseHandle(handle);
                if res.is_ok() {
                    return count;
                }
            }
        }
        0
    }

    /// Scans for leaking processes that exceed handle warning threshold (10,000 handles) to prevent kernel pool exhaustion.
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
