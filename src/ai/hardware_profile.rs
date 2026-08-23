// hardware_profile.rs
// Intelligent Hardware Tier Profiling & Adaptive Policy Engine.
// Dynamically detects system hardware constraints (Cores, Total RAM, Topology)
// and custom-tunes AI scheduling, memory thresholds, and I/O policies to guarantee
// buttery-smooth responsiveness even on low-end / older hardware.

use sysinfo::System;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HardwareTier {
    LowEndBudget,      // <= 4 Cores or <= 8GB RAM (Optimized for zero disk/CPU stutter)
    MidRangeStandard,  // 6-8 Cores and 12-16GB RAM
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
    #[must_use] 
    pub fn auto_detect(sys: &System) -> Self {
        let logical_cores = sys.cpus().len().max(1);
        let total_ram_mb = sys.total_memory() / (1024 * 1024);

        let tier = if logical_cores <= 4 || total_ram_mb <= 6144 {
            HardwareTier::LowEndBudget
        } else if logical_cores <= 8 || total_ram_mb <= 16384 {
            HardwareTier::MidRangeStandard
        } else {
            HardwareTier::HighEndEnthusiast
        };

        // RAM trim threshold: never go below 150MB on any tier to avoid shell component crashes
        let safe_trim_mb = match tier {
            HardwareTier::LowEndBudget => (total_ram_mb / 80).max(150), // ~1.25% of total RAM, min 150MB
            HardwareTier::MidRangeStandard => (total_ram_mb / 60).max(150), // ~1.6%
            HardwareTier::HighEndEnthusiast => (total_ram_mb / 40).max(200), // ~2.5%
        };

        match tier {
            HardwareTier::LowEndBudget => Self {
                tier,
                logical_cores,
                total_ram_mb,
                ram_pressure_trim_mb: safe_trim_mb,
                max_background_io_prio: 0, // VeryLow I/O
                enable_aggressive_io_throttle: true,
            },
            HardwareTier::MidRangeStandard => Self {
                tier,
                logical_cores,
                total_ram_mb,
                ram_pressure_trim_mb: safe_trim_mb,
                max_background_io_prio: 1, // Low I/O
                enable_aggressive_io_throttle: true,
            },
            HardwareTier::HighEndEnthusiast => Self {
                tier,
                logical_cores,
                total_ram_mb,
                ram_pressure_trim_mb: safe_trim_mb,
                max_background_io_prio: 1,
                enable_aggressive_io_throttle: false,
            },
        }
    }

    #[must_use] 
    pub fn is_low_end(&self) -> bool {
        self.tier == HardwareTier::LowEndBudget
    }

    /// For low-end PCs, high-precision timer (0.5ms) adds DPC overhead — use 1ms instead.
    #[must_use] 
    pub const fn optimal_timer_resolution_100ns(&self) -> u32 {
        match self.tier {
            HardwareTier::LowEndBudget => 10_000,    // 1.0ms — low overhead
            HardwareTier::MidRangeStandard => 7_500, // 0.75ms
            HardwareTier::HighEndEnthusiast => 5_000, // 0.5ms — maximum precision
        }
    }

    /// Emergency RAM trigger threshold — lower for PCs with less headroom.
    #[must_use] 
    pub const fn emergency_ram_threshold(&self) -> f32 {
        match self.tier {
            HardwareTier::LowEndBudget => 80.0, // trigger earlier on scarce RAM
            HardwareTier::MidRangeStandard => 87.0,
            HardwareTier::HighEndEnthusiast => 92.0,
        }
    }

    #[must_use] 
    pub const fn tier_name(&self) -> &'static str {
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
