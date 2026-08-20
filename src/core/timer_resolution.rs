use crate::utils::nt_api;

pub struct TimerResolutionManager {
    is_boosted: bool,
}

impl TimerResolutionManager {
    pub fn new() -> Self {
        Self { is_boosted: false }
    }

    /// Enables high-precision 0.5ms (5000 in 100ns units) timer resolution.
    pub fn enable_high_precision(&mut self) -> bool {
        if self.is_boosted {
            return true;
        }
        match nt_api::set_timer_resolution(5000) {
            Ok(_) => {
                self.is_boosted = true;
                true
            }
            Err(_) => false,
        }
    }

    /// Restores system default timer resolution (15.6ms).
    pub fn restore_default(&mut self) {
        if !self.is_boosted {
            return;
        }
        let _ = nt_api::set_timer_resolution(156250);
        self.is_boosted = false;
    }

    pub fn is_boosted(&self) -> bool {
        self.is_boosted
    }
}

impl Drop for TimerResolutionManager {
    fn drop(&mut self) {
        self.restore_default();
    }
}
