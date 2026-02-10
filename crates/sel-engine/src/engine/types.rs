//! Execution Types - Day 5 Foundation
//!
//! These types form the substrate for sovereign execution.
//! No logic, no I/O - pure type definitions.

use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use crate::engine::FactsLoggerError;

// ============================================================================
// EXECUTION REPORT
// ============================================================================

/// Complete execution report with all metrics and results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionReport {
    /// Mission hash (from canonicalization)
    pub mission_hash: String,
    
    /// Validation proof (from validator)
    pub validation_proof: String,
    
    /// Validator version that signed this mission
    pub validator_version: String,
    
    /// Workspace UUID used for isolation
    pub workspace_uuid: String,
    
    /// Total actions in mission
    pub actions_total: usize,
    
    /// Actions successfully executed
    pub actions_succeeded: usize,
    
    /// Actions that failed
    pub actions_failed: usize,
    
    /// Total execution duration in milliseconds
    pub total_duration_ms: u64,
    
    /// Path to facts file
    pub facts_file: PathBuf,
    
    /// Final hash from hash chain
    pub final_hash: String,
    
    /// Logical clock ticks consumed
    pub logical_ticks: u64,
    
    /// Workspace mode used
    pub workspace_mode: String,
}

impl ExecutionReport {
    /// Check if execution was fully successful
    pub fn is_success(&self) -> bool {
        self.actions_failed == 0 && self.actions_succeeded == self.actions_total
    }
    
    /// Get success rate as percentage
    pub fn success_rate(&self) -> f64 {
        if self.actions_total == 0 {
            return 100.0;
        }
        (self.actions_succeeded as f64 / self.actions_total as f64) * 100.0
    }
}

// ============================================================================
// ACTION RESULT
// ============================================================================

/// Result of executing a single action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionResult {
    /// Exit code (0 = success)
    pub exit_code: i32,
    
    /// Standard output captured
    pub stdout: String,
    
    /// Standard error captured
    pub stderr: String,
    
    /// Duration in milliseconds
    pub duration_ms: u64,
    
    /// Action index in mission
    pub action_index: usize,
}

impl ActionResult {
    /// Check if action succeeded
    pub fn is_success(&self) -> bool {
        self.exit_code == 0
    }
}

// ============================================================================
// EXECUTOR ERROR
// ============================================================================

/// All errors that can occur during execution
#[derive(Debug)]
pub enum ExecutorError {
    /// Workspace creation or management failed
    WorkspaceError(WorkspaceError),
    
    /// Capability violation detected
    CapabilityViolation(CapabilityViolation),
    
    /// Action execution failed
    ExecutionFailed {
        action_index: usize,
        command: String,
        reason: String,
    },
    
    /// Facts logging failed
    FactsLoggerError(FactsLoggerError),
    
    /// Mission parsing failed
    MissionParseError(String),
    
    /// Invalid mission structure
    InvalidMission(String),
    
    /// Timeout exceeded
    TimeoutExceeded {
        action_index: usize,
        limit_secs: u64,
        actual_secs: u64,
    },
    
    /// Memory limit exceeded
    MemoryExceeded {
        limit_mb: u64,
        actual_mb: u64,
    },
    
    /// Other errors
    Other(String),
}

impl std::fmt::Display for ExecutorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExecutorError::WorkspaceError(e) => {
                write!(f, "Workspace error: {}", e)
            }
            ExecutorError::CapabilityViolation(e) => {
                write!(f, "Capability violation: {}", e)
            }
            ExecutorError::ExecutionFailed { action_index, command, reason } => {
                write!(f, "Action {} failed ({}): {}", action_index, command, reason)
            }
            ExecutorError::FactsLoggerError(e) => {
                write!(f, "Facts logger error: {}", e)
            }
            ExecutorError::MissionParseError(e) => {
                write!(f, "Mission parse error: {}", e)
            }
            ExecutorError::InvalidMission(e) => {
                write!(f, "Invalid mission: {}", e)
            }
            ExecutorError::TimeoutExceeded { action_index, limit_secs, actual_secs } => {
                write!(f, "Action {} timeout: {}s limit, {}s actual", 
                    action_index, limit_secs, actual_secs)
            }
            ExecutorError::MemoryExceeded { limit_mb, actual_mb } => {
                write!(f, "Memory exceeded: {}MB limit, {}MB actual", limit_mb, actual_mb)
            }
            ExecutorError::Other(msg) => {
                write!(f, "Execution error: {}", msg)
            }
        }
    }
}

impl std::error::Error for ExecutorError {}

impl From<FactsLoggerError> for ExecutorError {
    fn from(e: FactsLoggerError) -> Self {
        ExecutorError::FactsLoggerError(e)
    }
}

impl From<WorkspaceError> for ExecutorError {
    fn from(e: WorkspaceError) -> Self {
        ExecutorError::WorkspaceError(e)
    }
}

// ============================================================================
// WORKSPACE ERROR
// ============================================================================

/// Errors related to workspace management
#[derive(Debug)]
pub enum WorkspaceError {
    /// Failed to create workspace directory
    CreationFailed(std::io::Error),
    
    /// Failed to set permissions
    PermissionsFailed(std::io::Error),
    
    /// Failed to cleanup workspace
    CleanupFailed(std::io::Error),
    
    /// Workspace path invalid
    InvalidPath(String),
    
    /// Workspace already exists
    AlreadyExists(PathBuf),
}

impl std::fmt::Display for WorkspaceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WorkspaceError::CreationFailed(e) => {
                write!(f, "Workspace creation failed: {}", e)
            }
            WorkspaceError::PermissionsFailed(e) => {
                write!(f, "Failed to set workspace permissions: {}", e)
            }
            WorkspaceError::CleanupFailed(e) => {
                write!(f, "Workspace cleanup failed: {}", e)
            }
            WorkspaceError::InvalidPath(p) => {
                write!(f, "Invalid workspace path: {}", p)
            }
            WorkspaceError::AlreadyExists(p) => {
                write!(f, "Workspace already exists: {}", p.display())
            }
        }
    }
}

impl std::error::Error for WorkspaceError {}

// ============================================================================
// CAPABILITY VIOLATION
// ============================================================================

/// Specific capability violations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CapabilityViolation {
    /// Binary execution not permitted
    BinaryExecutionDenied {
        command: String,
        action_index: usize,
    },
    
    /// File write not permitted
    FileWriteDenied {
        path: String,
        action_index: usize,
    },
    
    /// Network access not permitted
    NetworkAccessDenied {
        url: String,
        action_index: usize,
    },
    
    /// Execution time exceeded
    TimeoutExceeded {
        action_index: usize,
        limit_secs: u64,
    },
    
    /// Memory limit exceeded
    MemoryExceeded {
        action_index: usize,
        limit_mb: u64,
    },
    
    /// Workspace size exceeded
    WorkspaceSizeExceeded {
        limit_mb: u64,
        actual_mb: u64,
    },
}

impl std::fmt::Display for CapabilityViolation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CapabilityViolation::BinaryExecutionDenied { command, action_index } => {
                write!(f, "Action {}: Binary execution denied for '{}'", 
                    action_index, command)
            }
            CapabilityViolation::FileWriteDenied { path, action_index } => {
                write!(f, "Action {}: File write denied for '{}'", 
                    action_index, path)
            }
            CapabilityViolation::NetworkAccessDenied { url, action_index } => {
                write!(f, "Action {}: Network access denied for '{}'", 
                    action_index, url)
            }
            CapabilityViolation::TimeoutExceeded { action_index, limit_secs } => {
                write!(f, "Action {}: Timeout exceeded ({}s limit)", 
                    action_index, limit_secs)
            }
            CapabilityViolation::MemoryExceeded { action_index, limit_mb } => {
                write!(f, "Action {}: Memory exceeded ({}MB limit)", 
                    action_index, limit_mb)
            }
            CapabilityViolation::WorkspaceSizeExceeded { limit_mb, actual_mb } => {
                write!(f, "Workspace size exceeded: {}MB limit, {}MB actual", 
                    limit_mb, actual_mb)
            }
        }
    }
}

impl std::error::Error for CapabilityViolation {}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execution_report_is_success() {
        let report = ExecutionReport {
            mission_hash: "test".to_string(),
            validation_proof: "proof".to_string(),
            validator_version: "1.0.0".to_string(),
            workspace_uuid: "uuid".to_string(),
            actions_total: 5,
            actions_succeeded: 5,
            actions_failed: 0,
            total_duration_ms: 100,
            facts_file: PathBuf::from("/tmp/facts.jsonl"),
            final_hash: "hash".to_string(),
            logical_ticks: 10,
            workspace_mode: "ReadOnly".to_string(),
        };
        
        assert!(report.is_success());
        assert_eq!(report.success_rate(), 100.0);
    }

    #[test]
    fn test_action_result_is_success() {
        let result = ActionResult {
            exit_code: 0,
            stdout: "output".to_string(),
            stderr: "".to_string(),
            duration_ms: 50,
            action_index: 0,
        };
        
        assert!(result.is_success());
    }
}
