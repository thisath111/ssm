// ram_compressor.rs
// Aggressively flushes inactive processes from physical RAM to pagefile.
// Eliminates micro-stutters caused by RAM pressure during gaming.

use windows::Win32::System::Threading::{
    OpenProcess, PROCESS_SET_QUOTA, PROCESS_QUERY_INFORMATION,
};
use windows::Win32::System::Memory::SetProcessWorkingSetSizeEx;
use windows::Win32::Foundation::CloseHandle;
use sysinfo::{System, Pid};
use log::info;

const QUOTA_LIMITS_HARDWS_MIN_DISABLE: u32 = 0x00000002;

pub struct RamCompressor;

impl RamCompressor {
    pub fn new() -> Self {
        Self
    }

    /// Forces the Windows Memory Manager to trim the Working Set of a background
    /// process, pushing its pages to the pagefile and freeing physical RAM.
    /// This is identical to what EmptyWorkingSet() does internally.
    pub fn trim_process(&self, pid: u32) {
        unsafe {
            let access = PROCESS_SET_QUOTA | PROCESS_QUERY_INFORMATION;
            if let Ok(handle) = OpenProcess(access, false, pid) {
                // Setting min=-1, max=-1 tells Windows to trim working set
                // to the smallest possible size. Pages are lazily loaded back
                // from pagefile when accessed, causing zero data loss.
                let _ = SetProcessWorkingSetSizeEx(
                    handle,
                    usize::MAX,    // -1 = trim min
                    usize::MAX,    // -1 = trim max
                    windows::Win32::System::Memory::SETPROCESSWORKINGSETSIZEEX_FLAGS(QUOTA_LIMITS_HARDWS_MIN_DISABLE),
                );
                let _ = CloseHandle(handle);
            }
        }
    }

    /// Trims all non-critical background processes to free RAM for the foreground game.
    pub fn trim_all_background(&self, sys: &System, protected_pids: &[u32], safeguard: &crate::ai::safeguard::AiSafeguard) {
        let mut trimmed: u32 = 0;
        for (pid, process) in sys.processes() {
            let pid_u32 = pid.as_u32();
            if protected_pids.contains(&pid_u32) {
                continue;
            }

            let name = process.name().to_string_lossy().to_lowercase();

            // Never trim critical Windows processes
            if name.contains("system") || name.contains("winlogon") || 
               name.contains("csrss") || name.contains("smss") || 
               name.contains("lsass") || name.contains("services") {
                continue;
            }

            // Trim processes using >50MB RAM that are not the foreground app
            if process.memory() > 50 * 1024 * 1024 {
                if safeguard.authorize(crate::ai::safeguard::ActionType::TrimMemory, pid_u32, &name) {
                    self.trim_process(pid_u32);
                    trimmed += 1;
                }
            }
        }

        if trimmed > 0 {
            info!("[RamCompressor] Trimmed working set of {} background processes", trimmed);
        }
    }
}
