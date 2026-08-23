use crate::ai::intent_classifier::ProcessIntent;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SystemWorkloadState {
    PowerSaverIdle,
    StandardInteractive,
    CreatorDeveloperBoost,
    UltraGaming,
    EmergencyLoadShedding,
}

impl SystemWorkloadState {
    #[must_use] 
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::PowerSaverIdle => "Power-Efficient Idle",
            Self::StandardInteractive => "Standard Interactive",
            Self::CreatorDeveloperBoost => "Creator / Developer Turbo",
            Self::UltraGaming => "Ultra Gaming / Zero-Latency",
            Self::EmergencyLoadShedding => "Emergency Load-Shedding",
        }
    }
}

/// Dynamic Workload State Machine that synthesizes signals from CPU, Memory,
/// Foreground Process Intent, and GPU activity to determine the global operating mode.
pub struct WorkloadStateMachine {
    pub current_state: SystemWorkloadState,
    ticks_in_state: u64,
    /// RAM % threshold to trigger emergency load-shedding (hardware adaptive)
    pub emergency_ram_threshold: f32,
    /// CPU % threshold to trigger emergency load-shedding (hardware adaptive)
    pub emergency_cpu_threshold: f32,
}

impl Default for WorkloadStateMachine {
    fn default() -> Self {
        Self::new()
    }
}

impl WorkloadStateMachine {
    #[must_use] 
    pub const fn new() -> Self {
        Self {
            current_state: SystemWorkloadState::StandardInteractive,
            ticks_in_state: 0,
            emergency_ram_threshold: 92.0, // Default — overridden per hardware tier
            emergency_cpu_threshold: 95.0,
        }
    }

    /// Evaluates current metrics and transitions to the optimal system state.
    pub fn evaluate(
        &mut self,
        cpu_usage: f32,
        ram_usage: f32,
        foreground_intent: ProcessIntent,
        is_audio_active: bool,
    ) -> SystemWorkloadState {
        self.ticks_in_state += 1;

        let next_state = if ram_usage > self.emergency_ram_threshold
            || cpu_usage > self.emergency_cpu_threshold
        {
            SystemWorkloadState::EmergencyLoadShedding
        } else if foreground_intent == ProcessIntent::Gaming {
            SystemWorkloadState::UltraGaming
        } else if foreground_intent == ProcessIntent::CreativeWorkstation
            || foreground_intent == ProcessIntent::SoftwareDevelopment
            || (cpu_usage > 50.0 && !is_audio_active)
        {
            SystemWorkloadState::CreatorDeveloperBoost
        } else if cpu_usage < 5.0 && !is_audio_active && ram_usage < 50.0 {
            SystemWorkloadState::PowerSaverIdle
        } else {
            SystemWorkloadState::StandardInteractive
        };

        if next_state != self.current_state {
            self.current_state = next_state;
            self.ticks_in_state = 0;
        }

        self.current_state
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_machine_transitions() {
        let mut sm = WorkloadStateMachine::new();

        // 1. Gaming intent overrides to UltraGaming
        let state = sm.evaluate(25.0, 60.0, ProcessIntent::Gaming, true);
        assert_eq!(state, SystemWorkloadState::UltraGaming);

        // 2. High RAM triggers emergency load shedding
        let state_emerg = sm.evaluate(30.0, 95.0, ProcessIntent::Gaming, true);
        assert_eq!(state_emerg, SystemWorkloadState::EmergencyLoadShedding);

        // 3. Low CPU/RAM triggers PowerSaverIdle
        let state_idle = sm.evaluate(2.0, 30.0, ProcessIntent::InteractiveUi, false);
        assert_eq!(state_idle, SystemWorkloadState::PowerSaverIdle);

        // 4. Low-end hardware has lower emergency threshold
        let mut sm_lowend = WorkloadStateMachine::new();
        sm_lowend.emergency_ram_threshold = 80.0;
        sm_lowend.emergency_cpu_threshold = 90.0;
        let state_low = sm_lowend.evaluate(85.0, 82.0, ProcessIntent::InteractiveUi, false);
        assert_eq!(state_low, SystemWorkloadState::EmergencyLoadShedding);
    }
}
