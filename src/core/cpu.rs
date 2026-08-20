use windows::core::GUID;
use windows::Win32::System::Power::*;
use windows::Win32::System::Threading::{SetProcessAffinityMask, OpenProcess, PROCESS_SET_INFORMATION};
use windows::Win32::Foundation::CloseHandle;
use crate::sensors::cpu_topology::CpuTopology;

extern "system" {
    fn LocalFree(hmem: *mut std::ffi::c_void) -> *mut std::ffi::c_void;
}

const GUID_PROCESSOR_SETTINGS_SUBGROUP: GUID = GUID {
    data1: 0x54533251,
    data2: 0x82be,
    data3: 0x4824,
    data4: [0x96, 0xc1, 0x47, 0xb6, 0x0b, 0x74, 0x0d, 0x00],
};

const GUID_PROCESSOR_CORE_PARKING_MIN: GUID = GUID {
    data1: 0x0cc5b647,
    data2: 0xc1df,
    data3: 0x4637,
    data4: [0x89, 0x1a, 0xde, 0xc3, 0x5c, 0x31, 0x85, 0x83],
};

// Ultimate Performance Power Plan GUID
const GUID_ULTIMATE_PERFORMANCE: GUID = GUID {
    data1: 0xe9a42b02,
    data2: 0xd5df,
    data3: 0x448d,
    data4: [0xaa, 0x00, 0x03, 0xf1, 0x47, 0x49, 0xeb, 0x61],
};

// High Performance Power Plan GUID
const GUID_HIGH_PERFORMANCE: GUID = GUID {
    data1: 0x8c5e7fda,
    data2: 0xe8bf,
    data3: 0x4a96,
    data4: [0x9a, 0x85, 0x27, 0x0e, 0x06, 0x5d, 0x01, 0x1e],
};

pub struct CpuManager {
    topology: CpuTopology,
    original_power_scheme: Option<GUID>,
    is_boosted: bool,
}

impl CpuManager {
    pub fn new() -> Self {
        Self {
            topology: CpuTopology::detect(),
            original_power_scheme: None,
            is_boosted: false,
        }
    }

    pub fn topology(&self) -> &CpuTopology {
        &self.topology
    }

    /// Pin foreground process to Performance Cores (P-Cores) or all cores.
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

    /// Pin background CPU heavy process to Efficiency Cores (E-Cores) / secondary threads.
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

    /// Unparks all CPU cores and sets power plan to High/Ultimate Performance.
    pub fn enable_performance_mode(&mut self) {
        if self.is_boosted {
            return;
        }

        unsafe {
            let mut active_ptr: *mut GUID = std::ptr::null_mut();
            if PowerGetActiveScheme(None, &mut active_ptr).0 == 0 && !active_ptr.is_null() {
                self.original_power_scheme = Some(*active_ptr);
                let _ = LocalFree(active_ptr as *mut _);
            }

            // Try applying Ultimate Performance or High Performance scheme
            if PowerSetActiveScheme(None, Some(&GUID_ULTIMATE_PERFORMANCE)).0 != 0 {
                let _ = PowerSetActiveScheme(None, Some(&GUID_HIGH_PERFORMANCE));
            }

            // Unpark all cores by setting Core Parking Min to 100%
            if let Some(active_scheme) = self.original_power_scheme {
                let _ = PowerWriteACValueIndex(
                    None,
                    &active_scheme,
                    Some(&GUID_PROCESSOR_SETTINGS_SUBGROUP),
                    Some(&GUID_PROCESSOR_CORE_PARKING_MIN),
                    100,
                );
                let _ = PowerSetActiveScheme(None, Some(&active_scheme));
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
                let _ = PowerSetActiveScheme(None, Some(&original));
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
