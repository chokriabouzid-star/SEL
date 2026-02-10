//! Logical Clock for Deterministic Execution

use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

/// Logical clock combining wall clock reference and deterministic ticks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogicalClock {
    /// Mission epoch (wall clock reference)
    epoch_start: DateTime<Utc>,
    
    /// Event counter (deterministic)
    ticks: u64,
}

impl LogicalClock {
    /// Create new logical clock
    pub fn new() -> Self {
        Self {
            epoch_start: Utc::now(),
            ticks: 0,
        }
    }
    
    /// Tick the clock (increment counter)
    pub fn tick(&mut self) -> u64 {
        self.ticks += 1;
        self.ticks
    }
    
    /// Get current tick count
    pub fn ticks(&self) -> u64 {
        self.ticks
    }
    
    /// Get logical timestamp
    pub fn logical_timestamp(&self) -> String {
        format!("{}_tick_{}", 
            self.epoch_start.to_rfc3339(), 
            self.ticks)
    }
}

impl Default for LogicalClock {
    fn default() -> Self {
        Self::new()
    }
}
