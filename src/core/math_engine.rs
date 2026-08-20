/// High-Precision Proportional-Integral-Derivative (PID) Controller for dynamic workload management.
pub struct PidController {
    pub kp: f32,
    pub ki: f32,
    pub kd: f32,
    prev_error: f32,
    integral: f32,
    output_min: f32,
    output_max: f32,
}

impl PidController {
    pub fn new(kp: f32, ki: f32, kd: f32, output_min: f32, output_max: f32) -> Self {
        Self {
            kp,
            ki,
            kd,
            prev_error: 0.0,
            integral: 0.0,
            output_min,
            output_max,
        }
    }

    /// Computes the PID control output given setpoint and current measured value over dt.
    pub fn compute(&mut self, setpoint: f32, measured: f32, dt: f32) -> f32 {
        if dt <= 0.0 {
            return 0.0;
        }

        let error = setpoint - measured;
        self.integral = (self.integral + error * dt).clamp(self.output_min, self.output_max);
        let derivative = (error - self.prev_error) / dt;
        self.prev_error = error;

        let output = (self.kp * error) + (self.ki * self.integral) + (self.kd * derivative);
        output.clamp(self.output_min, self.output_max)
    }
}

/// 1D Kalman Filter for CPU & Memory workload state estimation.
pub struct KalmanPredictor {
    q: f32, // Process noise covariance
    r: f32, // Measurement noise covariance
    x: f32, // Estimated state
    p: f32, // Estimation error covariance
    k: f32, // Kalman gain
}

impl KalmanPredictor {
    pub fn new(process_noise: f32, measurement_noise: f32) -> Self {
        Self {
            q: process_noise,
            r: measurement_noise,
            x: 0.0,
            p: 1.0,
            k: 0.0,
        }
    }

    /// Filters raw measurement and returns optimal state estimate.
    pub fn update(&mut self, measurement: f32) -> f32 {
        // Prediction step
        self.p = self.p + self.q;

        // Update step
        self.k = self.p / (self.p + self.r);
        self.x = self.x + self.k * (measurement - self.x);
        self.p = (1.0 - self.k) * self.p;

        self.x
    }

    pub fn get_estimate(&self) -> f32 {
        self.x
    }
}

/// Page Fault Rate Calculus tracking derivatives d(PF)/dt and d²(PF)/dt².
pub struct PageFaultDerivator {
    prev_faults: u64,
    prev_rate: f32,
    pub first_derivative: f32,
    pub second_derivative: f32,
}

impl PageFaultDerivator {
    pub fn new() -> Self {
        Self {
            prev_faults: 0,
            prev_rate: 0.0,
            first_derivative: 0.0,
            second_derivative: 0.0,
        }
    }

    pub fn update(&mut self, current_faults: u64, dt_seconds: f32) {
        if dt_seconds <= 0.0 || self.prev_faults == 0 {
            self.prev_faults = current_faults;
            return;
        }

        let delta_faults = current_faults.saturating_sub(self.prev_faults);
        let current_rate = delta_faults as f32 / dt_seconds;

        self.first_derivative = current_rate;
        self.second_derivative = (current_rate - self.prev_rate) / dt_seconds;

        self.prev_faults = current_faults;
        self.prev_rate = current_rate;
    }
}
