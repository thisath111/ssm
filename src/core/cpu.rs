use crate::sensors::cpu_topology::CpuTopology;
use windows::core::GUID;
use windows::Win32::Foundation::CloseHandle;
use windows::Win32::System::Power::{
    PowerGetActiveScheme, PowerSetActiveScheme, PowerWriteACValueIndex,
};
use windows::Win32::System::Threading::{
    GetPriorityClass, OpenProcess, SetPriorityClass, SetProcessAffinityMask, HIGH_PRIORITY_CLASS,
    IDLE_PRIORITY_CLASS, PROCESS_QUERY_LIMITED_INFORMATION, PROCESS_SET_INFORMATION,
};

extern "system" {
    fn LocalFree(hmem: *mut std::ffi::c_void) -> *mut std::ffi::c_void;
}

use crate::utils::power_constants::*;

pub struct CpuManager {
    topology: CpuTopology,
    original_power_scheme: Option<GUID>,
    is_boosted: bool,
}

impl Default for CpuManager {
    fn default() -> Self {
        Self::new()
    }
}

impl CpuManager {
    #[must_use]
    pub fn new() -> Self {
        Self {
            topology: CpuTopology::detect(),
            original_power_scheme: None,
            is_boosted: false,
        }
    }

    #[must_use]
    pub const fn topology(&self) -> &CpuTopology {
        &self.topology
    }

    /// Pins foreground process to Performance Cores (P-Cores) or all cores.
    pub fn pin_foreground(&self, pid: u32) {
        if pid <= 4 || self.topology.p_core_mask == usize::MAX {
            return;
        }
        unsafe {
            if let Ok(handle) = OpenProcess(PROCESS_SET_INFORMATION, false, pid) {
                let _ = SetProcessAffinityMask(handle, self.topology.p_core_mask);
                let _ = CloseHandle(handle);
            }
        }
    }

    /// Pins background process to Efficiency Cores (E-Cores).
    pub fn pin_background(&self, pid: u32) {
        if pid <= 4 || self.topology.e_core_mask == usize::MAX {
            return;
        }
        unsafe {
            if let Ok(handle) = OpenProcess(PROCESS_SET_INFORMATION, false, pid) {
                let _ = SetProcessAffinityMask(handle, self.topology.e_core_mask);
                let _ = CloseHandle(handle);
            }
        }
    }

    /// Temporarily boosts a process to High Priority, returning its original priority.
    #[must_use]
    pub fn boost_process_priority(&self, pid: u32) -> Option<u32> {
        if pid <= 4 {
            return None;
        }
        unsafe {
            if let Ok(handle) = OpenProcess(
                PROCESS_SET_INFORMATION | PROCESS_QUERY_LIMITED_INFORMATION,
                false,
                pid,
            ) {
                let orig = GetPriorityClass(handle);
                let _ = SetPriorityClass(handle, HIGH_PRIORITY_CLASS);
                let _ = CloseHandle(handle);
                return Some(orig);
            }
        }
        None
    }

    /// Throttles a background process to Idle Priority.
    pub fn throttle_process_priority(&self, pid: u32) {
        if pid <= 4 {
            return;
        }
        unsafe {
            if let Ok(handle) = OpenProcess(PROCESS_SET_INFORMATION, false, pid) {
                let _ = SetPriorityClass(handle, IDLE_PRIORITY_CLASS);
                let _ = CloseHandle(handle);
            }
        }
    }

    /// Restores a process to its original Priority.
    pub fn restore_process_priority(&self, pid: u32, original_priority: u32) {
        if pid <= 4 || original_priority == 0 {
            return;
        }
        unsafe {
            if let Ok(handle) = OpenProcess(PROCESS_SET_INFORMATION, false, pid) {
                let _ = SetPriorityClass(
                    handle,
                    windows::Win32::System::Threading::PROCESS_CREATION_FLAGS(original_priority),
                );
                let _ = CloseHandle(handle);
            }
        }
    }

    /// Unparks CPU cores and applies High/Ultimate Performance plan.
    pub fn enable_performance_mode(&mut self) {
        if self.is_boosted {
            return;
        }

        unsafe {
            let mut active_ptr: *mut GUID = std::ptr::null_mut();
            if PowerGetActiveScheme(None, &raw mut active_ptr).0 == 0 && !active_ptr.is_null() {
                self.original_power_scheme = Some(*active_ptr);
                let _ = LocalFree(active_ptr.cast());
            }

            // Apply Ultimate or High Performance scheme
            if PowerSetActiveScheme(None, Some(&GUID_ULTIMATE_PERFORMANCE)).0 != 0 {
                let _ = PowerSetActiveScheme(None, Some(&GUID_HIGH_PERFORMANCE));
            }

            // Unpark all cores (Core Parking Min = 100%)
            if let Some(active_scheme) = self.original_power_scheme {
                let _ = PowerWriteACValueIndex(
                    None,
                    &raw const active_scheme,
                    Some(&GUID_PROCESSOR_SETTINGS_SUBGROUP),
                    Some(&GUID_PROCESSOR_CORE_PARKING_MIN),
                    100,
                );
                // Suppress C-States (Idle Disable) to eliminate micro-stutters
                let _ = PowerWriteACValueIndex(
                    None,
                    &raw const active_scheme,
                    Some(&GUID_PROCESSOR_SETTINGS_SUBGROUP),
                    Some(&GUID_PROCESSOR_IDLE_DISABLE),
                    1,
                );
                let _ = PowerSetActiveScheme(None, Some(&raw const active_scheme));
            }
        }
        self.is_boosted = true;
    }

    /// Restores original power scheme on completion.
    pub fn restore_default_mode(&mut self) {
        if !self.is_boosted {
            return;
        }
        if let Some(original) = self.original_power_scheme {
            unsafe {
                let _ = PowerSetActiveScheme(None, Some(&raw const original));
            }
        }
        self.is_boosted = false;
    }
}

impl Drop for CpuManager {
    fn drop(&mut self) {
        self.restore_default_mode();
    }
}
