use log::warn;
use windows::Win32::System::Threading::{
    AvRevertMmThreadCharacteristics, AvSetMmThreadCharacteristicsW,
};

pub struct MmcssThreadGuard {
    task_handle: Option<windows::Win32::Foundation::HANDLE>,
}

impl MmcssThreadGuard {
    /// Registers the current thread with the Multimedia Class Scheduler Service (MMCSS).
    /// "Pro Audio" or "Games" tasks get a massive priority boost and guarantee against preemption.
    #[must_use]
    pub fn new(task_name: &str) -> Self {
        let mut task_index = 0;
        let mut task_name_wide: Vec<u16> =
            task_name.encode_utf16().chain(std::iter::once(0)).collect();

        unsafe {
            if let Ok(handle) = AvSetMmThreadCharacteristicsW(
                windows::core::PWSTR(task_name_wide.as_mut_ptr()),
                &raw mut task_index,
            ) {
                return Self {
                    task_handle: Some(handle),
                };
            } else {
                warn!(
                    "Failed to register thread with MMCSS for task: {}",
                    task_name
                );
            }
        }
        Self { task_handle: None }
    }
}

impl Drop for MmcssThreadGuard {
    fn drop(&mut self) {
        if let Some(handle) = self.task_handle {
            unsafe {
                let _ = AvRevertMmThreadCharacteristics(handle);
            }
        }
    }
}
