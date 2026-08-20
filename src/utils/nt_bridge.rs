// nt_bridge.rs
// BUG FIX: Fixed OOB read vulnerability and incomplete buffer parsing.

use ntapi::ntexapi::{NtQuerySystemInformation, SystemProcessInformation, SYSTEM_PROCESS_INFORMATION};

pub struct ProcessInfo {
    pub pid: u32,
    pub thread_count: u32,
    pub handle_count: u32,
    pub page_faults: u32,
    pub working_set_size: u64,
}

#[allow(dead_code)] // Unused currently, but preserved safely
pub fn scan_processes_nt() -> Vec<ProcessInfo> {
    let mut results = Vec::new();
    let mut buf_size = 512 * 1024;
    let mut buffer: Vec<u8> = Vec::with_capacity(buf_size as usize);
    let mut actual: u32 = 0;
    let mut success = false;

    for _ in 0..4 {
        buffer.resize(buf_size as usize, 0);
        let status = unsafe {
            NtQuerySystemInformation(
                SystemProcessInformation,
                buffer.as_mut_ptr() as *mut _,
                buf_size,
                &mut actual,
            )
        };

        if status == 0 { // STATUS_SUCCESS
            success = true;
            break;
        }
        if status == 0xC0000004u32 as i32 { // STATUS_INFO_LENGTH_MISMATCH
            buf_size = actual + 16384;
        } else {
            return results;
        }
    }

    // BUG FIX: Only parse if we actually succeeded in getting the data
    if !success {
        return results;
    }

    let mut ptr = buffer.as_ptr();
    let end = unsafe { buffer.as_ptr().add(buffer.len()) };

    loop {
        // BUG FIX: Strict bounds checking to prevent OOB read
        if ptr as usize + std::mem::size_of::<SYSTEM_PROCESS_INFORMATION>() > end as usize {
            break;
        }

        let spi = unsafe { &*(ptr as *const SYSTEM_PROCESS_INFORMATION) };
        
        let pid = spi.UniqueProcessId as usize as u32;
        let thread_count = spi.NumberOfThreads;
        let handle_count = spi.HandleCount;
        let page_faults = spi.PageFaultCount;
        let working_set_size = spi.WorkingSetSize as u64;

        results.push(ProcessInfo {
            pid,
            thread_count,
            handle_count,
            page_faults,
            working_set_size,
        });

        if spi.NextEntryOffset == 0 {
            break;
        }
        
        // BUG FIX: Validate NextEntryOffset before advancing
        let next_offset = spi.NextEntryOffset as usize;
        if next_offset < std::mem::size_of::<SYSTEM_PROCESS_INFORMATION>() {
            break; // Malformed data
        }

        ptr = unsafe { ptr.add(next_offset) };
        if ptr >= end {
            break; // OOB
        }
    }

    results
}
