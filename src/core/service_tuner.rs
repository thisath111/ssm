use std::process::Command;
use std::os::windows::process::CommandExt;

const CREATE_NO_WINDOW: u32 = 0x08000000;

pub struct ServiceTuner {
    pub is_paused: bool,
}

impl ServiceTuner {
    pub fn new() -> Self {
        Self { is_paused: false }
    }

    /// Temporarily pauses or throttles non-essential lag-inducing background services during high workload.
    pub fn pause_background_services(&mut self) {
        if self.is_paused {
            return;
        }

        let services = ["SysMain", "WSearch", "DiagTrack", "MapsBroker"];
        for svc in services {
            let _ = Command::new("sc")
                .args(&["stop", svc])
                .creation_flags(CREATE_NO_WINDOW)
                .spawn();
        }
        
        self.is_paused = true;
    }

    /// Restores Windows background services to normal operation.
    pub fn restore_background_services(&mut self) {
        if !self.is_paused {
            return;
        }

        let services = ["SysMain", "WSearch"];
        for svc in services {
            let _ = Command::new("sc")
                .args(&["start", svc])
                .creation_flags(CREATE_NO_WINDOW)
                .spawn();
        }

        self.is_paused = false;
    }
}
