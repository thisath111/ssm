use sysinfo::{System};
use std::time::Instant;

pub struct ThermalSensor {
    high_load_start: Option<Instant>,
    stress_level: f32, // 0.0 to 1.0
}

impl ThermalSensor {
    pub fn new() -> Self {
        Self {
            high_load_start: None,
            stress_level: 0.0,
        }
    }

    pub fn update(&mut self, sys: &System) {
        let load = sys.global_cpu_usage();
        
        if load > 85.0 {
            if self.high_load_start.is_none() {
                self.high_load_start = Some(Instant::now());
            } else if let Some(start) = self.high_load_start {
                let duration = start.elapsed().as_secs();
                // If under heavy load for > 60 seconds, thermal stress is assumed high
                self.stress_level = (duration as f32 / 60.0).clamp(0.0, 1.0);
            }
        } else {
            self.high_load_start = None;
            self.stress_level = (self.stress_level - 0.05).max(0.0); // Cooldown
        }
    }

    pub fn get_stress_level(&self) -> f32 {
        self.stress_level
    }
    
    pub fn is_throttling_risk(&self) -> bool {
        self.stress_level > 0.8
    }
}
