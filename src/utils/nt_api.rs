use ntapi::ntpsapi::{
    NtResumeProcess, NtSetInformationProcess, NtSuspendProcess, ProcessIoPriority,
};
use windows::Win32::Foundation::HANDLE;

extern "system" {
    pub fn NtSetTimerResolution(
        DesiredResolution: u32,
        SetResolution: u8,
        ActualResolution: *mut u32,
    ) -> i32;

    pub fn NtQueryTimerResolution(
        MinimumResolution: *mut u32,
        MaximumResolution: *mut u32,
        CurrentResolution: *mut u32,
    ) -> i32;

    pub fn NtSetSystemInformation(
        SystemInformationClass: u32,
        SystemInformation: *mut std::ffi::c_void,
        SystemInformationLength: u32,
    ) -> i32;
}

const SYSTEM_MEMORY_LIST_INFORMATION: u32 = 80;
const PURGE_STANDBY_LIST: u32 = 4;
const PROCESS_PAGE_PRIORITY: u32 = 39;

/// Sets global Windows timer resolution in 100ns units (e.g. 5000 = 0.5ms).
pub fn set_timer_resolution(desired_100ns: u32) -> Result<u32, i32> {
    let mut actual: u32 = 0;
    let status = unsafe { NtSetTimerResolution(desired_100ns, 1, &raw mut actual) };
    if status >= 0 {
        Ok(actual)
    } else {
        Err(status)
    }
}

/// Queries current timer resolution ranges (min, max, current) in 100ns units.
#[must_use] 
pub fn query_timer_resolution() -> Option<(u32, u32, u32)> {
    let mut min: u32 = 0;
    let mut max: u32 = 0;
    let mut current: u32 = 0;
    let status = unsafe { NtQueryTimerResolution(&raw mut min, &raw mut max, &raw mut current) };
    if status >= 0 {
        Some((min, max, current))
    } else {
        None
    }
}

/// Sets Process I/O Priority (0 = Very Low, 1 = Low, 2 = Normal, 3 = High).
#[must_use] 
pub fn set_process_io_priority(handle: HANDLE, priority: u32) -> bool {
    let mut io_prio = priority;
    let status = unsafe {
        NtSetInformationProcess(
            handle.0.cast(),
            ProcessIoPriority,
            (&raw mut io_prio).cast(),
            std::mem::size_of::<u32>() as u32,
        )
    };
    status >= 0
}

/// Sets Process Memory Page Priority (1 = Lowest, 5 = Normal/High).
#[must_use] 
pub fn set_process_page_priority(handle: HANDLE, priority: u32) -> bool {
    let mut page_prio = priority;
    let status = unsafe {
        NtSetInformationProcess(
            handle.0.cast(),
            PROCESS_PAGE_PRIORITY as _,
            (&raw mut page_prio).cast(),
            std::mem::size_of::<u32>() as u32,
        )
    };
    status >= 0
}

/// Safely purges Windows Standby Memory List.
#[must_use] 
pub fn purge_standby_list() -> bool {
    let mut command: u32 = PURGE_STANDBY_LIST;
    let status = unsafe {
        NtSetSystemInformation(
            SYSTEM_MEMORY_LIST_INFORMATION,
            (&raw mut command).cast(),
            std::mem::size_of::<u32>() as u32,
        )
    };
    status >= 0
}

/// Suspends process execution using `NtSuspendProcess`.
#[must_use] 
pub fn suspend_process_nt(handle: HANDLE) -> bool {
    let status = unsafe { NtSuspendProcess(handle.0.cast()) };
    status >= 0
}

/// Resumes process execution using `NtResumeProcess`.
#[must_use] 
pub fn resume_process_nt(handle: HANDLE) -> bool {
    let status = unsafe { NtResumeProcess(handle.0.cast()) };
    status >= 0
}
