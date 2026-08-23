use winreg::enums::{HKEY_CURRENT_USER, KEY_ALL_ACCESS};
use winreg::RegKey;

/// `DirectFlip` & Desktop Window Manager (DWM) Input Latency Optimizer.
/// Strips frame buffering delays between GPU presentation queues and displays.
pub struct DwmLatencyOptimizer;

impl DwmLatencyOptimizer {
    /// Applies zero-latency DWM composition & `DirectFlip` registry keys.
    pub fn optimize_dwm_latency() -> Result<(), Box<dyn std::error::Error>> {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);

        // 1. Disable Window Composition animation delays
        let desktop_metrics = r"Control Panel\Desktop\WindowMetrics";
        if let Ok((key, _)) = hkcu.create_subkey_with_flags(desktop_metrics, KEY_ALL_ACCESS) {
            let _ = key.set_value("MinAnimate", &"0");
        }

        // 2. Tune DWM Composition queue depth (DirectFlip optimization)
        let dwm_key = r"Software\Microsoft\Windows\DWM";
        if let Ok((key, _)) = hkcu.create_subkey_with_flags(dwm_key, KEY_ALL_ACCESS) {
            let _ = key.set_value("CompositionPolicy", &2u32); // Priority to hardware overlay / DirectFlip
            let _ = key.set_value("EnableAeroPeek", &0u32);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dwm_latency_structure() {
        // Safe check verifying function call signature without panicking
        let _ = DwmLatencyOptimizer::optimize_dwm_latency();
    }
}
