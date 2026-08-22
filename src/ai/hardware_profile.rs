// hardware_profile.rs
// Intelligent Hardware Tier Profiling & Adaptive Policy Engine.
// Dynamically detects system hardware constraints (Cores, Total RAM, Topology)
// and custom-tunes AI scheduling, memory thresholds, and I/O policies to guarantee
// buttery-smooth responsiveness even on low-end / older hardware.

use sysinfo::System;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HardwareTier {
    LowEndBudget,    // <= 4 Cores or <= 8GB RAM (Optimized for zero disk/CPU stutter)
    MidRangeStandard, // 6-8 Cores and 12-16GB RAM
    HighEndEnthusiast, // > 8 Cores and > 16GB RAM
}

#[derive(Debug, Clone)]
pub struct HardwareProfile {
    pub tier: HardwareTier,
    pub logical_cores: usize,
    pub total_ram_mb: u64,
    pub ram_pressure_trim_mb: u64,
    pub max_background_io_prio: u32,
    pub enable_aggressive_io_throttle: bool,
}

impl HardwareProfile {
    /// Auto-detects hardware topology from the operating system.
    pub fn auto_detect(sys: &System) -> Self {
        let logical_cores = sys.cpus().len().max(1);
        let total_ram_mb = sys.total_memory() / (1024 * 1024);

        let tier = if logical_cores <= 4 || total_ram_mb <= 8192 {
            HardwareTier::LowEndBudget
        } else if logical_cores <= 8 && total_ram_mb <= 16384 {
            HardwareTier::MidRangeStandard
        } else {
            HardwareTier::HighEndEnthusiast
        };

        match tier {
            HardwareTier::LowEndBudget => Self {
                tier,
                logical_cores,
                total_ram_mb,
                ram_pressure_trim_mb: 60, // Trim non-critical apps > 60MB to save scarce RAM
                max_background_io_prio: 0, // VeryLow I/O (0) for background apps so HDD/SSD never locks
                enable_aggressive_io_throttle: true,
            },
            HardwareTier::MidRangeStandard => Self {
                tier,
                logical_cores,
                total_ram_mb,
                ram_pressure_trim_mb: 100,
                max_background_io_prio: 1, // Low I/O (1)
                enable_aggressive_io_throttle: true,
            },
            HardwareTier::HighEndEnthusiast => Self {
                tier,
                logical_cores,
                total_ram_mb,
                ram_pressure_trim_mb: 200,
                max_background_io_prio: 1,
                enable_aggressive_io_throttle: false,
            },
        }
    }

    pub fn is_low_end(&self) -> bool {
        self.tier == HardwareTier::LowEndBudget
    }

    pub fn tier_name(&self) -> &'static str {
        match self.tier {
            HardwareTier::LowEndBudget => "Budget / Legacy PC (Ultra-Smooth Smoothness Mode)",
            HardwareTier::MidRangeStandard => "Standard Balanced Performance Profile",
            HardwareTier::HighEndEnthusiast => "High-End Overdrive Profile",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hardware_profile_detection() {
        let mut sys = System::new_all();
        sys.refresh_all();
        let profile = HardwareProfile::auto_detect(&sys);
        assert!(profile.logical_cores >= 1);
        assert!(profile.total_ram_mb > 0);
        assert!(!profile.tier_name().is_empty());
    }
}
