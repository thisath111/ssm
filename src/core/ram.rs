use std::collections::HashSet;
use sysinfo::System;
use windows::Win32::System::Threading::{OpenProcess, PROCESS_SET_QUOTA, PROCESS_QUERY_INFORMATION};
use windows::Win32::System::Memory::{SetProcessWorkingSetSizeEx, SETPROCESSWORKINGSETSIZEEX_FLAGS};
use windows::Win32::Foundation::CloseHandle;
use crate::utils::nt_api;
use crate::sensors::ram_pressure::{RamPressureSensor, RamPressureLevel};

const QUOTA_LIMITS_HARDWS_MIN_DISABLE: u32 = 0x00000002;

pub struct RamManager {
    pub sensor: RamPressureSensor,
    last_trim_tick: u64,
    trimmed_pids: HashSet<u32>,
}

impl RamManager {
    pub fn new() -> Self {
        Self {
            sensor: RamPressureSensor::new(),
            last_trim_tick: 0,
            trimmed_pids: HashSet::new(),
        }
    }

    /// Purges Standby List RAM without dumping active app memory to pagefile.
    pub fn purge_standby_memory(&self) -> bool {
        nt_api::purge_standby_list()
    }

    /// Trims physical working set for background processes safely under sustained RAM pressure.
    pub fn trim_background_working_sets(&mut self, sys: &System, protected_pids: &[u32], tick_count: u64) -> u32 {
        if tick_count.saturating_sub(self.last_trim_tick) < 20 {
            return 0;
        }

        if self.sensor.level < RamPressureLevel::Critical {
            return 0;
        }

        self.last_trim_tick = tick_count;
        let mut trimmed_count = 0;

        let critical_processes = [
            "system", "registry", "smss", "csrss", "wininit", "winlogon",
            "lsass", "services", "dwm", "audiodg", "explorer", "ssm",
        ];

        for (pid, process) in sys.processes() {
            let pid_u32 = pid.as_u32();
            if protected_pids.contains(&pid_u32) || pid_u32 <= 4 {
                continue;
            }

            let name = process.name().to_string_lossy().to_lowercase();
            if critical_processes.iter().any(|&c| name.contains(c)) {
                continue;
            }

            if process.memory() > 100 * 1024 * 1024 {
                if self.trim_single_process(pid_u32) {
                    trimmed_count += 1;
                    self.trimmed_pids.insert(pid_u32);
                }
            }
        }

        trimmed_count
    }

    /// Safe single-process working set trim.
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
