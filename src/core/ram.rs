use crate::sensors::ram_pressure::{RamPressureLevel, RamPressureSensor};
use crate::utils::nt_api;
use std::collections::HashSet;
use sysinfo::System;
use windows::Win32::Foundation::CloseHandle;
use windows::Win32::System::Memory::{
    SetProcessWorkingSetSizeEx, SETPROCESSWORKINGSETSIZEEX_FLAGS,
};
use windows::Win32::System::Threading::{
    OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_SET_QUOTA,
};

const QUOTA_LIMITS_HARDWS_MIN_DISABLE: u32 = 0x00000002;

pub struct RamManager {
    pub sensor: RamPressureSensor,
    last_trim_tick: u64,
    trimmed_pids: HashSet<u32>,
}

impl Default for RamManager {
    fn default() -> Self {
        Self::new()
    }
}

impl RamManager {
    #[must_use]
    pub fn new() -> Self {
        Self {
            sensor: RamPressureSensor::new(),
            last_trim_tick: 0,
            trimmed_pids: HashSet::new(),
        }
    }

    /// Purges Standby List RAM without dumping active app memory to pagefile.
    #[must_use]
    pub fn purge_standby_memory(&self) -> bool {
        nt_api::purge_standby_list()
    }

    /// Trims physical working set for background processes safely under sustained RAM pressure.
    pub fn trim_background_working_sets(
        &mut self,
        sys: &System,
        protected_pids: &[u32],
        tick_count: u64,
        min_memory_mb: u64,
    ) -> u32 {
        if tick_count.saturating_sub(self.last_trim_tick) < 20 {
            return 0;
        }

        if self.sensor.level < RamPressureLevel::Critical {
            return 0;
        }

        self.last_trim_tick = tick_count;
        let mut trimmed_count = 0;
        let min_bytes = min_memory_mb * 1024 * 1024;

        for (pid, process) in sys.processes() {
            let pid_u32 = pid.as_u32();
            if protected_pids.contains(&pid_u32) || pid_u32 <= 4 {
                continue;
            }

            let name = process.name().to_string_lossy();
            if crate::core::stability_shield::StabilityShield::is_immune(pid_u32, &name) {
                continue;
            }

            if process.memory() > min_bytes && self.trim_single_process(pid_u32) {
                trimmed_count += 1;
                self.trimmed_pids.insert(pid_u32);
            }
        }

        trimmed_count
    }

    /// Safe single-process working set trim.
    #[must_use]
    pub fn trim_single_process(&self, pid: u32) -> bool {
        unsafe {
            let access = PROCESS_SET_QUOTA | PROCESS_QUERY_INFORMATION;
            if let Ok(handle) = OpenProcess(access, false, pid) {
                let status = SetProcessWorkingSetSizeEx(
                    handle,
                    usize::MAX,
                    usize::MAX,
                    SETPROCESSWORKINGSETSIZEEX_FLAGS(QUOTA_LIMITS_HARDWS_MIN_DISABLE),
                );
                let _ = CloseHandle(handle);
                return status.is_ok();
            }
        }
        false
    }
}
