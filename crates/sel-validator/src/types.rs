//! # SEL Validator Types
//! SEL Core 1.0 - FULLY DETERMINISTIC
//! 🔴 NO Utc::now(), NO randomness

use serde::{Serialize, Deserialize};
use sel_common::SEL_CORE_VERSION;

/// A validated action that passed sovereign validation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ValidatedAction {
    pub command: String,
    pub args: Vec<String>,
}

/// Cryptographic proof of validation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ValidationProof(pub String);

/// Workspace access mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorkspaceMode {
    ReadOnly,
    ReadWrite,
}

/// Execution capabilities granted to a validated mission
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExecutionCapabilities {
    pub workspace_mode: WorkspaceMode,
    pub allowed_commands: Vec<String>,
    pub max_actions: Option<usize>,
}

impl Default for ExecutionCapabilities {
    fn default() -> Self {
        Self {
            workspace_mode: WorkspaceMode::ReadOnly,
            allowed_commands: vec!["echo".to_string(), "pwd".to_string()],
            max_actions: Some(1000),
        }
    }
}

/// A mission that has passed sovereign validation
/// 🔴 DETERMINISTIC: No timestamps, no randomness
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatedMission {
    pub core_version: String,
    pub capabilities: ExecutionCapabilities,
    pub validation_proof: ValidationProof,
    /// 🔴 REMOVED: validated_at - كان يستخدم Utc::now() وغير حتمي
    pub validator_version: String,
    pub actions: Vec<ValidatedAction>,
    pub mission_hash: String,
}

impl ValidatedMission {
    pub fn new(
        capabilities: ExecutionCapabilities,
        proof: String,
    ) -> Self {
        Self {
            core_version: SEL_CORE_VERSION.to_string(),
            capabilities,
            validation_proof: ValidationProof(proof),
            validator_version: crate::VALIDATOR_VERSION.to_string(),
            actions: Vec::new(),
            mission_hash: String::new(),
        }
    }
    
    pub fn new_with_actions(
        capabilities: ExecutionCapabilities,
        proof: String,
        actions: Vec<ValidatedAction>,
    ) -> Self {
        let mut mission = Self::new(capabilities, proof);
        mission.actions = actions;
        mission
    }
    
    pub fn validator_version(&self) -> &str {
        &self.validator_version
    }
    
    pub fn workspace_mode(&self) -> WorkspaceMode {
        self.capabilities.workspace_mode
    }
    
    pub fn capabilities(&self) -> &ExecutionCapabilities {
        &self.capabilities
    }
    
    pub fn mission_hash(&self) -> String {
        self.mission_hash.clone()
    }
    
    pub fn set_mission_hash(&mut self, hash: String) {
        self.mission_hash = hash;
    }
    
    pub fn validation_proof_str(&self) -> &str {
        &self.validation_proof.0
    }
    
    pub fn actions(&self) -> Vec<ValidatedAction> {
        self.actions.clone()
    }
}
