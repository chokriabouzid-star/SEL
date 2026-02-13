//! # Sovereign Resource Limits
//! SEL Core 1.0 - Mandatory resource governance
//! These limits are NOT optional for Core compliance.

/// Resource limits enforced by SEL Core 1.0
/// All limits are in accordance with SEL Standard 1.0
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResourceLimits {
    /// Maximum number of actions per mission
    /// Core 1.0 default: 1000
    pub max_actions: usize,
    
    /// Maximum logical ticks per execution
    /// Core 1.0 default: 10,000
    pub max_ticks: u64,
    
    /// Maximum stdout bytes per mission
    /// Core 1.0 default: 1,048,576 (1 MiB)
    pub max_stdout_bytes: usize,
    
    /// Maximum stderr bytes per mission
    /// Core 1.0 default: 102,400 (100 KiB)
    pub max_stderr_bytes: usize,
    
    /// Maximum facts logged per mission
    /// Core 1.0 default: 10,000
    pub max_facts: usize,
}

impl Default for ResourceLimits {
    /// SEL Core 1.0 default limits
    /// These values are part of the SEL Standard specification
    fn default() -> Self {
        Self {
            max_actions: 1000,
            max_ticks: 10_000,
            max_stdout_bytes: 1_048_576,  // 1 MiB
            max_stderr_bytes: 102_400,    // 100 KiB
            max_facts: 10_000,
        }
    }
}

impl ResourceLimits {
    /// Create new ResourceLimits with custom values
    /// Use with caution - custom limits may affect SEL Core compliance
    pub fn new(
        max_actions: usize,
        max_ticks: u64,
        max_stdout_bytes: usize,
        max_stderr_bytes: usize,
        max_facts: usize,
    ) -> Self {
        Self {
            max_actions,
            max_ticks,
            max_stdout_bytes,
            max_stderr_bytes,
            max_facts,
        }
    }
    
    /// Core 1.0 compliant limits
    /// Use this to ensure SEL Core compliance
    pub fn core_compliant() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_limits_core_compliant() {
        let limits = ResourceLimits::default();
        assert_eq!(limits.max_actions, 1000);
        assert_eq!(limits.max_ticks, 10_000);
        assert_eq!(limits.max_stdout_bytes, 1_048_576);
        assert_eq!(limits.max_stderr_bytes, 102_400);
        assert_eq!(limits.max_facts, 10_000);
    }
    
    #[test]
    fn test_core_compliant_constructor() {
        let limits = ResourceLimits::core_compliant();
        assert_eq!(limits.max_actions, 1000);
    }
}
