// power_injector.rs
// Manages CPU core parking and power scheme optimizations.
// Unparks all cores when gaming mode is active to reduce wake-up latency.

use windows::core::GUID;
use windows::Win32::System::Power::*;
use log::{info, warn};

// Direct FFI binding to avoid crate version/feature path differences for LocalFree
extern "system" {
    fn LocalFree(hmem: *mut std::ffi::c_void) -> *mut std::ffi::c_void;
}

use crate::utils::power_constants::*;

pub struct PowerInjector {
    original_min_cores: Option<u32>,
    original_max_cores: Option<u32>,
    gaming_mode_active: bool,
}

impl PowerInjector {
    pub fn new() -> Self {
        Self {
            original_min_cores: None,
            original_max_cores: None,
            gaming_mode_active: false,
        }
    }

    /// Retrieves the current active power plan's GUID.
    fn get_active_scheme(&self) -> Option<GUID> {
        unsafe {
            let mut active_ptr: *mut GUID = std::ptr::null_mut();
            let status = PowerGetActiveScheme(None, &mut active_ptr);
            if status.0 == 0 && !active_ptr.is_null() {
                let scheme = *active_ptr;
                // Free the allocated memory using standard Win32 LocalFree
                let _ = LocalFree(active_ptr as *mut std::ffi::c_void);
                Some(scheme)
            } else {
                warn!("[PowerInjector] Failed to query active power scheme. Status: {}", status.0);
                None
            }
        }
    }

    /// Backs up original values and sets core parking limits to 100% (unparks all CPU cores).
    pub fn enable_gaming_mode(&mut self) {
        if self.gaming_mode_active {
            return;
        }

        let scheme = match self.get_active_scheme() {
            Some(s) => s,
            None => return,
        };

        unsafe {
            // Backup current min core parking value
            let mut min_val: u32 = 0;
            let status_min = PowerReadACValueIndex(
                None,
                Some(&scheme),
                Some(&GUID_PROCESSOR_SETTINGS_SUBGROUP),
                Some(&GUID_PROCESSOR_CORE_PARKING_MIN),
                &mut min_val,
            );
            if status_min.0 == 0 {
                self.original_min_cores = Some(min_val);
            }

            // Backup current max core parking value
            let mut max_val: u32 = 0;
            let status_max = PowerReadACValueIndex(
                None,
                Some(&scheme),
                Some(&GUID_PROCESSOR_SETTINGS_SUBGROUP),
                Some(&GUID_PROCESSOR_CORE_PARKING_MAX),
                &mut max_val,
            );
            if status_max.0 == 0 {
                self.original_max_cores = Some(max_val);
            }

            // Set both min and max to 100 to unpark all cores
            let s1 = PowerWriteACValueIndex(
                None,
                &scheme,
                Some(&GUID_PROCESSOR_SETTINGS_SUBGROUP),
                Some(&GUID_PROCESSOR_CORE_PARKING_MIN),
                100,
            );
            let s2 = PowerWriteACValueIndex(
                None,
                &scheme,
                Some(&GUID_PROCESSOR_SETTINGS_SUBGROUP),
                Some(&GUID_PROCESSOR_CORE_PARKING_MAX),
                100,
            );

            if s1.0 == 0 && s2.0 == 0 {
                // Apply the active scheme to refresh the settings instantly
                let s_apply = PowerSetActiveScheme(None, Some(&scheme));
                if s_apply.0 == 0 {
                    info!("[PowerInjector] Unparked all CPU cores successfully (100% min/max active).");
                    self.gaming_mode_active = true;
                } else {
                    warn!("[PowerInjector] Failed to apply power scheme: {}", s_apply.0);
                }
            } else {
                warn!("[PowerInjector] Failed to write core parking indices: {} / {}", s1.0, s2.0);
            }
        }
    }

    /// Restores the original core parking values.
    pub fn disable_gaming_mode(&mut self) {
        if !self.gaming_mode_active {
            return;
        }

        let scheme = match self.get_active_scheme() {
            Some(s) => s,
            None => return,
        };

        unsafe {
            let mut restored = false;

            if let Some(orig_min) = self.original_min_cores {
                let s1 = PowerWriteACValueIndex(
                    None,
                    &scheme,
                    Some(&GUID_PROCESSOR_SETTINGS_SUBGROUP),
                    Some(&GUID_PROCESSOR_CORE_PARKING_MIN),
                    orig_min,
                );
                if s1.0 == 0 {
                    restored = true;
                }
            }

            if let Some(orig_max) = self.original_max_cores {
                let s2 = PowerWriteACValueIndex(
                    None,
                    &scheme,
                    Some(&GUID_PROCESSOR_SETTINGS_SUBGROUP),
                    Some(&GUID_PROCESSOR_CORE_PARKING_MAX),
                    orig_max,
                );
                if s2.0 == 0 {
                    restored = true;
                }
            }

            if restored {
                let _ = PowerSetActiveScheme(None, Some(&scheme));
                info!("[PowerInjector] Restored original core parking profile successfully.");
            }

            self.gaming_mode_active = false;
        }
    }
}

impl Drop for PowerInjector {
    fn drop(&mut self) {
        self.disable_gaming_mode();
    }
}
