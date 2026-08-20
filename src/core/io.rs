// io.rs
// BUG FIX: NtSetInformationProcess return value is now checked and logged on error.

use ntapi::ntpsapi::{NtSetInformationProcess, ProcessIoPriority};
use windows::Win32::System::Threading::{OpenProcess, PROCESS_SET_INFORMATION};
use windows::Win32::Foundation::CloseHandle;
use log::warn;

pub struct IoOptimizer;

impl IoOptimizer {
    /// Lowers I/O priority of a background process to Very Low (0) to reduce disk contention.
    pub fn set_io_priority_very_low(pid: u32) {
        unsafe {
            if let Ok(handle) = OpenProcess(PROCESS_SET_INFORMATION, false, pid) {
                let mut io_prio: u32 = 0; // IoPriorityVeryLow = 0
                let status = NtSetInformationProcess(
                    handle.0 as _,
                    ProcessIoPriority,
                    &mut io_prio as *mut _ as *mut _,
                    std::mem::size_of::<u32>() as u32,
                );
                // BUG FIX: Log if the NT call fails
                if status < 0 {
                    warn!("[IoOptimizer] set_io_priority_very_low failed for PID {}: NTSTATUS={:#010x}", pid, status as u32);
                }
                let _ = CloseHandle(handle);
            }
        }
    }

    /// Restores I/O priority to Normal (2) when gaming ends.
    pub fn set_io_priority_normal(pid: u32) {
        unsafe {
            if let Ok(handle) = OpenProcess(PROCESS_SET_INFORMATION, false, pid) {
                let mut io_prio: u32 = 2; // IoPriorityNormal = 2
                let status = NtSetInformationProcess(
                    handle.0 as _,
                    ProcessIoPriority,
                    &mut io_prio as *mut _ as *mut _,
                    std::mem::size_of::<u32>() as u32,
                );
                if status < 0 {
                    warn!("[IoOptimizer] set_io_priority_normal failed for PID {}: NTSTATUS={:#010x}", pid, status as u32);
                }
                let _ = CloseHandle(handle);
            }
        }
    }
}
