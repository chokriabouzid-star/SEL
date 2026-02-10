//! Day 4 Types - Sovereign Validation

use serde::{Deserialize, Serialize};
use std::time::Duration;
use chrono::{DateTime, Utc};

/// Semantic version for validator
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SemanticVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl std::fmt::Display for SemanticVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl SemanticVersion {
    pub const fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self { major, minor, patch }
    }
}

/// Current validator version
pub const VALIDATOR_VERSION: SemanticVersion = SemanticVersion::new(1, 0, 0);

/// Workspace execution mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorkspaceMode {
    /// Strict sandbox
    Strict,
    /// Permissive mode (for development)
    Permissive,
    /// Read-only operations
    ReadOnly,
}

/// Execution capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionCapabilities {
    pub allowed_commands: Vec<String>,
    pub allowed_paths: Vec<String>,
    pub max_execution_time: Duration,
    pub workspace_mode: WorkspaceMode,
}

/// Validated mission with proof
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatedMission {
    validator_version: SemanticVersion,
    validation_proof: String,
    workspace_mode: WorkspaceMode,
    capabilities: ExecutionCapabilities,
    validated_at: DateTime<Utc>,
}

impl ValidatedMission {
    pub fn new(
        capabilities: ExecutionCapabilities,
        proof: String,
    ) -> Self {
        Self {
            validator_version: VALIDATOR_VERSION,
            validation_proof: proof,
            workspace_mode: capabilities.workspace_mode,
            capabilities,
            validated_at: Utc::now(),
        }
    }

    pub fn validator_version(&self) -> &SemanticVersion {
        &self.validator_version
    }

    pub fn validation_proof(&self) -> &str {
        &self.validation_proof
    }

    pub fn workspace_mode(&self) -> WorkspaceMode {
        self.workspace_mode
    }

    pub fn capabilities(&self) -> &ExecutionCapabilities {
        &self.capabilities
    }
    
    pub fn validated_at(&self) -> &DateTime<Utc> {
        &self.validated_at
    }
}

/// Proof generation error
#[derive(Debug, thiserror::Error)]
pub enum ProofError {
    #[error("Hash generation failed: {0}")]
    HashFailed(String),
    #[error("Serialization failed: {0}")]
    SerializationFailed(String),
}
