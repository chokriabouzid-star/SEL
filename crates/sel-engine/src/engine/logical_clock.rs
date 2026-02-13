//! # Logical Clock - Deterministic Time
//! SEL Core 1.0 - No wall time, only logical ticks
//! 
//! 🔴 ممنوع استخدام timestamp في hash chain
//! ✅ فقط logical_tick هو المصدر المسموح به

/// Deterministic logical clock
/// SEL Core 1.0: Only source of ordering in execution
#[derive(Debug, Clone)]
pub struct LogicalClock {
    /// Logical tick counter (يبدأ من 0)
    ticks: u64,
}

impl LogicalClock {
    /// Create new logical clock
    pub fn new() -> Self {
        Self {
            ticks: 0,
        }
    }
    
    /// Increment tick counter
    pub fn tick(&mut self) -> u64 {
        self.ticks += 1;
        self.ticks
    }
    
    /// Get current tick count
    pub fn ticks(&self) -> u64 {
        self.ticks
    }
}

impl Default for LogicalClock {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_logical_clock() {
        let mut clock = LogicalClock::new();
        assert_eq!(clock.ticks(), 0);
        
        clock.tick();
        assert_eq!(clock.ticks(), 1);
        
        clock.tick();
        assert_eq!(clock.ticks(), 2);
    }
}
