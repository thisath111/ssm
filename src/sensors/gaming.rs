use crate::utils::win32;
use sysinfo::{Pid, System};

pub struct GamingSensor {
    known_game_signatures: Vec<&'static str>,
}

impl Default for GamingSensor {
    fn default() -> Self {
        Self::new()
    }
}

impl GamingSensor {
    #[must_use]
    pub fn new() -> Self {
        Self {
            known_game_signatures: vec![
                "unreal", "unity", "engine", "shipping", "win64", "launcher",
            ],
        }
    }

    /// Determines if a full-screen game or graphics intensive app is active in foreground.
    #[must_use]
    pub fn is_gaming_active(&self, sys: &System) -> bool {
        let foreground_hwnd = match win32::get_foreground_hwnd() {
            Some(h) => h,
            None => return false,
        };

        let pid = win32::get_process_id_from_hwnd(foreground_hwnd);
        if pid == 0 || pid <= 4 {
            return false;
        }

        let is_fullscreen = win32::is_fullscreen(foreground_hwnd);

        if let Some(process) = sys.process(Pid::from_u32(pid)) {
            let name = process.name().to_string_lossy().to_lowercase();
            let exe_path = process
                .exe()
                .map(|p| p.to_string_lossy().to_lowercase())
                .unwrap_or_default();

            // Exclude desktop shell and browsers when playing fullscreen videos
            if name.contains("explorer")
                || name.contains("chrome")
                || name.contains("edge")
                || name.contains("firefox")
                || name.contains("vlc")
                || name.contains("mpc-hc")
            {
                return false;
            }

            // Behavioral Heuristic: Is it running fullscreen and using heavy RAM/CPU?
            // High memory and CPU + Fullscreen is highly likely a game or renderer
            let mem_mb = process.memory() / (1024 * 1024);
            if is_fullscreen && mem_mb > 500 {
                return true;
            }

            for sig in &self.known_game_signatures {
                if name.contains(sig) || exe_path.contains(sig) {
                    return true;
                }
            }
        }

        false
    }
}
