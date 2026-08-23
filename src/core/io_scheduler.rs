use crate::utils::nt_api;
use windows::Win32::Foundation::CloseHandle;
use windows::Win32::System::Threading::{OpenProcess, PROCESS_SET_INFORMATION};

pub struct IoScheduler;

impl IoScheduler {
    /// Sets process I/O priority (0 = Very Low, 1 = Low, 2 = Normal, 3 = High).
    #[must_use] 
    pub fn set_io_priority(pid: u32, priority: u32) -> bool {
        if pid <= 4 {
            return false;
        }
        unsafe {
            if let Ok(handle) = OpenProcess(PROCESS_SET_INFORMATION, false, pid) {
                let ok = nt_api::set_process_io_priority(handle, priority);
                let _ = CloseHandle(handle);
                return ok;
            }
        }
        false
    }

    /// Sets process memory page priority (1 = Lowest, 5 = Normal/High).
    #[must_use] 
    pub fn set_page_priority(pid: u32, priority: u32) -> bool {
        if pid <= 4 {
            return false;
        }
        unsafe {
            if let Ok(handle) = OpenProcess(PROCESS_SET_INFORMATION, false, pid) {
                let ok = nt_api::set_process_page_priority(handle, priority);
                let _ = CloseHandle(handle);
                return ok;
            }
        }
        false
    }

    /// Sets background CPU hog to lowest I/O priority (0) and page priority (1).
    pub fn deprioritize_background_process(pid: u32) {
        Self::set_io_priority(pid, 0);
        Self::set_page_priority(pid, 1);
    }

    /// Boosts foreground application to high I/O priority (3) and page priority (5).
    pub fn prioritize_foreground_process(pid: u32) {
        Self::set_io_priority(pid, 3);
        Self::set_page_priority(pid, 5);
    }

    /// Restores a backgrounded process to normal I/O priority (2) and page priority (5).
    pub fn restore_process_io(pid: u32) {
        Self::set_io_priority(pid, 2);
        Self::set_page_priority(pid, 5);
    }
}
