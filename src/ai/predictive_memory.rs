/// Predictive Standby Memory Forecaster.
/// Uses dynamic linear trend estimation & rate-of-change momentum to predict
/// upcoming memory pressure spikes before they cause paging stalls.
pub struct PredictiveMemoryForecaster {
    history: [f32; 10],
    index: usize,
    count: usize,
    pub predicted_usage_in_5s: f32,
    pub momentum: f32,
}

impl PredictiveMemoryForecaster {
    pub fn new() -> Self {
        Self {
            history: [0.0; 10],
            index: 0,
            count: 0,
            predicted_usage_in_5s: 0.0,
            momentum: 0.0,
        }
    }

    /// Feeds the latest memory usage percentage (0.0 to 100.0) and updates 5-second prediction.
    pub fn record_and_predict(&mut self, current_used_percent: f32) -> f32 {
        self.history[self.index] = current_used_percent;
        self.index = (self.index + 1) % self.history.len();
        if self.count < self.history.len() {
            self.count += 1;
        }

        if self.count < 3 {
            self.predicted_usage_in_5s = current_used_percent;
            return current_used_percent;
        }

        // Calculate simple linear velocity (momentum) over the available window
        let oldest_idx = if self.count < self.history.len() {
            0
        } else {
            self.index
        };
        let oldest = self.history[oldest_idx];
        let dt = (self.count - 1) as f32; // seconds (assuming 1s ticks)

        let velocity = (current_used_percent - oldest) / dt;
        self.momentum = velocity;

        // Extrapolate 5 seconds ahead with exponential smoothing dampening
        let predicted = current_used_percent + (velocity * 5.0);
        self.predicted_usage_in_5s = predicted.clamp(0.0, 100.0);
        self.predicted_usage_in_5s
    }

    /// Returns true if a memory crisis is imminent (predicted > 85% or rapid surge > 3% / sec).
    pub fn should_preemptively_purge(&self, current_used_percent: f32) -> bool {
        (self.predicted_usage_in_5s >= 85.0 && current_used_percent >= 70.0)
            || (self.momentum > 3.0 && current_used_percent >= 65.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_predictive_memory_forecaster() {
        let mut forecaster = PredictiveMemoryForecaster::new();

        // Simulate steady memory
        forecaster.record_and_predict(50.0);
        forecaster.record_and_predict(50.0);
        forecaster.record_and_predict(50.0);
        assert!(!forecaster.should_preemptively_purge(50.0));

        // Simulate rapid memory surge (e.g. game level loading)
        forecaster.record_and_predict(55.0);
        forecaster.record_and_predict(65.0);
        forecaster.record_and_predict(75.0);

        // Momentum should be positive and predicted usage should cross critical threshold
        assert!(forecaster.momentum > 0.0);
        assert!(forecaster.should_preemptively_purge(75.0));
    }
}
