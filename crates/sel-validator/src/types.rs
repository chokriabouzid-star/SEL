//! # SEL Validator Types
//! SEL Core 1.2 - HMAC + Ed25519 dual-proof types

use sel_common::SEL_VERSION;
use serde::{Deserialize, Serialize};

/// A validated action that passed sovereign validation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ValidatedAction {
    pub command: String,
    pub args: Vec<String>,
}

/// Cryptographic proof of validation (HMAC only)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ValidationProof(pub String);

impl ValidationProof {
    pub fn new(signature: String) -> Self {
        Self(signature)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatedMission {
    pub core_version: String,
    pub capabilities: ExecutionCapabilities,
    pub validation_proof: ValidationProof,
    /// Ed25519 signature (hex) over the canonical JSON.
    /// `None` until `.with_ed25519(...)` is called.
    #[serde(default)]
    pub ed25519_proof: Option<String>,
    /// Hex-encoded Ed25519 public key matching `ed25519_proof`.
    #[serde(default)]
    pub ed25519_public_key: Option<String>,
    pub validator_version: String,
    pub actions: Vec<ValidatedAction>,
    pub mission_hash: String,
    /// Whether strict security rules (path-traversal, dangerous-pattern
    /// checks) were applied during validation. This field is included in
    /// the signed payload so a verifier can distinguish a fully-checked
    /// proof from one produced with --no-strict.
    pub strict_mode: bool,
}

impl ValidatedMission {
    pub fn new(capabilities: ExecutionCapabilities, proof: ValidationProof) -> Self {
        Self {
            core_version: SEL_VERSION.to_string(),
            capabilities,
            validation_proof: proof,
            ed25519_proof: None,
            ed25519_public_key: None,
            validator_version: crate::VALIDATOR_VERSION.to_string(),
            actions: Vec::new(),
            mission_hash: String::new(),
            strict_mode: true,
        }
    }

    pub fn new_with_actions(
        capabilities: ExecutionCapabilities,
        proof: ValidationProof,
        actions: Vec<ValidatedAction>,
    ) -> Self {
        let mut mission = Self::new(capabilities, proof);
        mission.actions = actions;
        mission
    }

    /// Attach Ed25519 proof + public key (builder-style, additive).
    pub fn with_ed25519(mut self, proof_hex: String, public_key_hex: String) -> Self {
        self.ed25519_proof = Some(proof_hex);
        self.ed25519_public_key = Some(public_key_hex);
        self
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

    pub fn set_strict_mode(&mut self, strict: bool) {
        self.strict_mode = strict;
    }

    pub fn strict_mode(&self) -> bool {
        self.strict_mode
    }

    pub fn validation_proof(&self) -> &ValidationProof {
        &self.validation_proof
    }

    pub fn validation_proof_str(&self) -> &str {
        &self.validation_proof.0
    }

    pub fn ed25519_proof_str(&self) -> Option<&str> {
        self.ed25519_proof.as_deref()
    }

    pub fn ed25519_public_key_str(&self) -> Option<&str> {
        self.ed25519_public_key.as_deref()
    }

    pub fn actions(&self) -> Vec<ValidatedAction> {
        self.actions.clone()
    }
}
