use std::process::Command;
use std::os::windows::process::CommandExt;

const CREATE_NO_WINDOW: u32 = 0x08000000;

pub struct ServiceTuner;

impl ServiceTuner {
    /// Temporarily pauses or throttles non-essential lag-inducing background services during high workload.
    pub fn pause_background_services() {
        let services = ["SysMain", "WSearch", "DiagTrack", "MapsBroker"];
        for svc in services {
            let _ = Command::new("sc")
                .args(&["stop", svc])
                .creation_flags(CREATE_NO_WINDOW)
                .output();
        }
    }

    /// Restores Windows background services to normal operation.
    pub fn restore_background_services() {
        let services = ["SysMain", "WSearch"];
        for svc in services {
            let _ = Command::new("sc")
                .args(&["start", svc])
                .creation_flags(CREATE_NO_WINDOW)
                .output();
        }
    }
}
