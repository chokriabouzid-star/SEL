//! SEL Engine Core Types
//! SEL Core 1.0 Compliant

use std::path::PathBuf;
use serde::{Serialize, Deserialize};

/// Result of executing a single action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionResult {
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
    pub duration_ms: u64,
    pub action_index: usize,
}

/// Complete execution report for a mission
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionReport {
    pub mission_hash: String,
    pub validation_proof: String,
    pub validator_version: String,
    pub workspace_uuid: String,
    pub actions_total: usize,
    pub actions_succeeded: usize,
    pub actions_failed: usize,
    pub total_duration_ms: u64,
    pub facts_file: PathBuf,
    pub final_hash: String,
    pub logical_ticks: u64,
    pub workspace_mode: String,
    pub stdout_bytes: usize,
    pub stderr_bytes: usize,
}

/// Legacy error type - maintained for compatibility
/// New code should use SovereignError from sel_common
#[derive(Debug)]
pub enum ExecutorError {
    WorkspaceError(String),
    CapabilityViolation(String),
    ExecutionFailed(String),
    Internal(String),
}

impl std::fmt::Display for ExecutorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExecutorError::WorkspaceError(msg) => write!(f, "Workspace error: {}", msg),
            ExecutorError::CapabilityViolation(msg) => write!(f, "Capability violation: {}", msg),
            ExecutorError::ExecutionFailed(msg) => write!(f, "Execution failed: {}", msg),
            ExecutorError::Internal(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for ExecutorError {}
