use std::collections::HashSet;
use windows::Win32::System::Threading::{OpenProcess, PROCESS_SUSPEND_RESUME};
use windows::Win32::Foundation::CloseHandle;
use ntapi::ntpsapi::{NtSuspendProcess, NtResumeProcess};

pub struct ProcessSuspender {
    suspended_pids: HashSet<u32>,
}

impl ProcessSuspender {
    pub fn new() -> Self {
        Self {
            suspended_pids: HashSet::new(),
        }
    }

    pub fn suspend_process(&mut self, pid: u32) {
        if self.suspended_pids.contains(&pid) {
            return;
        }

        unsafe {
            if let Ok(handle) = OpenProcess(PROCESS_SUSPEND_RESUME, false, pid) {
                let status = NtSuspendProcess(handle.0 as _);
                if status >= 0 {
                    self.suspended_pids.insert(pid);
                }
                let _ = CloseHandle(handle);
            }
        }
    }

    pub fn resume_process(&mut self, pid: u32) {
        if !self.suspended_pids.contains(&pid) {
            return;
        }

        unsafe {
            if let Ok(handle) = OpenProcess(PROCESS_SUSPEND_RESUME, false, pid) {
                NtResumeProcess(handle.0 as _);
                let _ = CloseHandle(handle);
            }
        }
        self.suspended_pids.remove(&pid);
    }

    pub fn resume_all(&mut self) {
        let pids: Vec<u32> = self.suspended_pids.iter().copied().collect();
        for pid in pids {
            self.resume_process(pid);
        }
    }
}

impl Drop for ProcessSuspender {
    fn drop(&mut self) {
        self.resume_all();
    }
}
