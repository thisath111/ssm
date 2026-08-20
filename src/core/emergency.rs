// emergency.rs
// Nuclear option: when the system is about to completely freeze,
// mass-suspend all non-essential processes and force-trim all working sets.
// This is the last line of defense before the user has to hard-reset.

use sysinfo::System;
use crate::core::suspend::ProcessSuspender;
use crate::core::ram_compressor::RamCompressor;
use crate::sensors::ram_pressure::RamPressureLevel;
use log::{warn, info};

pub struct EmergencyMode {
    is_active: bool,
    consecutive_emergency_ticks: u8,
    ram_compressor: RamCompressor,
}

impl EmergencyMode {
    pub fn new() -> Self {
        Self {
            is_active: false,
            consecutive_emergency_ticks: 0,
            ram_compressor: RamCompressor::new(),
        }
    }

    /// Evaluate whether to trigger emergency mode.
    /// Called every tick from the engine.
    pub fn evaluate(
        &mut self,
        sys: &System,
        ram_level: RamPressureLevel,
        cpu_usage: f32,
        suspender: &mut ProcessSuspender,
        foreground_pid: u32,
        safeguard: &crate::ai::safeguard::AiSafeguard,
    ) {
        let is_emergency = ram_level == RamPressureLevel::Emergency
            || (cpu_usage > 98.0 && ram_level >= RamPressureLevel::Critical);

        if is_emergency {
            self.consecutive_emergency_ticks = self.consecutive_emergency_ticks.saturating_add(1);
        } else {
            if self.is_active && !is_emergency {
                // Crisis is over — resume
                info!("[Emergency] System stabilized — resuming suspended processes");
                suspender.resume_all();
                self.is_active = false;
            }
            self.consecutive_emergency_ticks = 0;
            return;
        }

        // Trigger after 5 consecutive emergency ticks (2.5 seconds of crisis)
        if self.consecutive_emergency_ticks >= 5 && !self.is_active {
            warn!("[Emergency] ⚠️ EMERGENCY MODE ACTIVATED — mass suspending background processes!");
            self.activate(sys, suspender, foreground_pid, safeguard);
        }
    }

    fn activate(&mut self, sys: &System, suspender: &mut ProcessSuspender, foreground_pid: u32, safeguard: &crate::ai::safeguard::AiSafeguard) {
        self.is_active = true;

        let self_pid = std::process::id();

        // Critical processes that must NEVER be touched
        let untouchable = [
            "system", "registry", "smss", "csrss", "wininit", "winlogon",
            "lsass", "services", "svchost", "dwm", "explorer", "fontdrvhost",
            "audiodg", "ctfmon", "sihost", "taskhostw", "runtimebroker",
            "smart-system-manager", "smart_system_manager",
        ];

        let mut suspended_count: u32 = 0;
        let mut trimmed_count: u32 = 0;

        for (pid, process) in sys.processes() {
            let pid_u32 = pid.as_u32();

            // Never touch foreground, self, or system PID 0/4
            if pid_u32 == foreground_pid || pid_u32 == self_pid || pid_u32 <= 4 {
                continue;
            }

            let name = process.name().to_string_lossy().to_lowercase();

            // Never touch critical Windows processes
            if untouchable.iter().any(|&u| name.contains(u)) {
                // But DO trim their working sets to free RAM
                self.ram_compressor.trim_process(pid_u32);
                trimmed_count += 1;
                continue;
            }

            let mem = process.memory();
            let cpu = process.cpu_usage();

            // Suspend anything using meaningful resources
            if cpu > 1.0 || mem > 20 * 1024 * 1024 {
                if safeguard.authorize(crate::ai::safeguard::ActionType::Suspend, pid_u32, &name) {
                    suspender.suspend_process(pid_u32);
                    suspended_count += 1;
                }
                if safeguard.authorize(crate::ai::safeguard::ActionType::TrimMemory, pid_u32, &name) {
                    self.ram_compressor.trim_process(pid_u32);
                }
            } else {
                // At minimum, trim working set
                if safeguard.authorize(crate::ai::safeguard::ActionType::TrimMemory, pid_u32, &name) {
                    self.ram_compressor.trim_process(pid_u32);
                    trimmed_count += 1;
                }
            }
        }

        warn!(
            "[Emergency] Suspended {} processes, trimmed {} processes",
            suspended_count, trimmed_count
        );
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }
}
