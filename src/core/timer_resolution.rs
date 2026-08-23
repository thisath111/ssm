use crate::utils::nt_api;

pub struct TimerResolutionManager {
    is_boosted: bool,
}

impl Default for TimerResolutionManager {
    fn default() -> Self {
        Self::new()
    }
}

impl TimerResolutionManager {
    #[must_use] 
    pub const fn new() -> Self {
        Self { is_boosted: false }
    }

    /// Enables high-precision 0.5ms (5000 in 100ns units) timer resolution.
    pub fn enable_high_precision(&mut self) -> bool {
        self.enable_with_resolution(5_000)
    }

    /// Enables hardware-appropriate timer resolution to avoid DPC overhead on low-end PCs.
    pub fn enable_adaptive(&mut self, resolution_100ns: u32) -> bool {
        self.enable_with_resolution(resolution_100ns)
    }

    fn enable_with_resolution(&mut self, resolution_100ns: u32) -> bool {
        if self.is_boosted {
            return true;
        }
        match nt_api::set_timer_resolution(resolution_100ns) {
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

    #[must_use] 
    pub const fn is_boosted(&self) -> bool {
        self.is_boosted
    }
}

impl Drop for TimerResolutionManager {
    fn drop(&mut self) {
        self.restore_default();
    }
}
