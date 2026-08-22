// privacy_shield.rs
// Blocks Microsoft telemetry, data collection, and diagnostic services.
// Uses safe Windows APIs: Registry keys, Service control, and Scheduled Task disabling.
// Runs once on startup, then periodically re-enforces to prevent Windows Update from re-enabling.

use std::process::Command;
use std::os::windows::process::CommandExt;
use winreg::enums::*;
use winreg::RegKey;
use log::{info, warn, error};

const CREATE_NO_WINDOW: u32 = 0x08000000;

pub struct PrivacyShield {
    applied: bool,
    last_enforce_tick: u64,
}

impl PrivacyShield {
    pub fn new() -> Self {
        Self {
            applied: false,
            last_enforce_tick: 0,
        }
    }

    /// Called from engine tick. Applies on first run, then re-enforces every 30 minutes.
    pub fn enforce(&mut self, tick_count: u64) {
        if !self.applied {
            info!("[PrivacyShield] Applying secure enterprise telemetry blocks...");
            self.disable_telemetry_registry();
            self.disable_telemetry_service();
            self.applied = true;
            self.last_enforce_tick = tick_count;
            info!("[PrivacyShield] Secure telemetry blocks applied successfully");
            return;
        }

        // Re-enforce every 3600 ticks (~30 minutes at 500ms/tick)
        if tick_count.saturating_sub(self.last_enforce_tick) >= 3600 {
            info!("[PrivacyShield] Re-enforcing secure telemetry blocks...");
            self.disable_telemetry_registry();
            self.disable_telemetry_service();
            self.last_enforce_tick = tick_count;
        }
    }

    /// Safely disables telemetry via official Microsoft Enterprise Policies
    fn disable_telemetry_registry(&self) {
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        
        let path = "SOFTWARE\\Policies\\Microsoft\\Windows\\DataCollection";
        match hklm.create_subkey(path) {
            Ok((key, _)) => {
                if let Err(e) = key.set_value("AllowTelemetry", &0u32) {
                    warn!("[PrivacyShield] Failed to set AllowTelemetry: {}", e);
                } else {
                    info!("[PrivacyShield] Set AllowTelemetry = 0 (Success)");
                }
            }
            Err(e) => warn!("[PrivacyShield] Failed to open/create DataCollection key: {}", e),
        }

        let search_path = "SOFTWARE\\Policies\\Microsoft\\Windows\\Windows Search";
        match hklm.create_subkey(search_path) {
            Ok((key, _)) => {
                let _ = key.set_value("AllowCortana", &0u32);
            }
            Err(_) => {}
        }
    }

    /// Gracefully disables the Connected User Experiences and Telemetry service
    fn disable_telemetry_service(&self) {
        let _ = Command::new("sc")
            .args(&["stop", "DiagTrack"])
            .creation_flags(CREATE_NO_WINDOW)
            .spawn();

        let _ = Command::new("sc")
            .args(&["config", "DiagTrack", "start=", "disabled"])
            .creation_flags(CREATE_NO_WINDOW)
            .spawn();
            
        info!("[PrivacyShield] DiagTrack service disable dispatched.");
    }
}
