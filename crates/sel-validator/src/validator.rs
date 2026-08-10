//! # Sovereign Validator Implementation
//! SEL Core 1.0 - HMAC only, no Ed25519

use sel_common::{canonicalize_json, ResourceKind, SelResult, SovereignError};
use serde_json::{from_str, Value};
use sha2::{Digest, Sha256};

use crate::{
    crypto_authority::CryptoAuthority,
    rules::{validate_security_rules, SecurityViolation},
    types::{ExecutionCapabilities, ValidatedAction, ValidatedMission, ValidationProof},
};

const ALLOWED_COMMANDS: [&str; 2] = ["echo", "pwd"];

#[derive(Debug, Clone)]
pub struct ValidationConfig {
    pub max_actions: usize,
    pub strict_mode: bool,
}

impl Default for ValidationConfig {
    fn default() -> Self {
        Self {
            max_actions: 1000,
            strict_mode: true,
        }
    }
}

pub struct Validator {
    config: ValidationConfig,
    crypto: CryptoAuthority,
}

impl Validator {
    pub fn new(config: ValidationConfig) -> Self {
        let crypto =
            CryptoAuthority::from_env_or_generate(&CryptoAuthority::default_key_path())
                .unwrap_or_else(|e| panic!("SEL: invalid HMAC key configuration: {}", e));
        Self { config, crypto }
    }

    pub fn validate(&self, mission_json: &str) -> SelResult<ValidatedMission> {
        // Canonicalization
        let canonical = canonicalize_json(mission_json)
            .map_err(|e| SovereignError::InvalidMissionFormat(e.to_string()))?;

        // Generate deterministic mission hash
        let mut hasher = Sha256::new();
        hasher.update(canonical.as_bytes());
        let mission_hash = hex::encode(hasher.finalize());

        let parsed: Value = from_str(&canonical)
            .map_err(|e| SovereignError::InvalidMissionFormat(e.to_string()))?;

        let actions = self.extract_actions(&parsed)?;

        // ✅ COMMAND WHITELIST - يحدث أولاً
        for action in &actions {
            if !ALLOWED_COMMANDS.contains(&action.command.as_str()) {
                return Err(SovereignError::CapabilityViolation(
                    format!("Command not allowed in SEL Core 1.0: '{}'. Only 'echo' and 'pwd' are permitted.", 
                        action.command)
                ));
            }
        }

        // Enforce max_actions
        if actions.len() > self.config.max_actions {
            return Err(SovereignError::ResourceExhaustion {
                kind: ResourceKind::Actions,
                limit: self.config.max_actions as u64,
                requested: actions.len() as u64,
            });
        }

        // ✅ SECURITY RULES - path traversal and dangerous patterns only
        if self.config.strict_mode {
            if let Some(violation) = validate_security_rules(&actions) {
                return match violation {
                    SecurityViolation::PathTraversal(path) => {
                        Err(SovereignError::WorkspaceViolation(path))
                    }
                    SecurityViolation::DangerousPattern(pattern) => {
                        Err(SovereignError::ValidationFailed(format!(
                            "Dangerous pattern detected: {}",
                            pattern
                        )))
                    }
                };
            }
        }

        // Generate HMAC proof
        let signature = self.crypto.sign(&canonical);
        let proof = ValidationProof::new(signature);

        // Create ValidatedMission
        let mut validated =
            ValidatedMission::new_with_actions(ExecutionCapabilities::default(), proof, actions);

        validated.set_mission_hash(mission_hash);

        Ok(validated)
    }

    fn extract_actions(&self, parsed: &Value) -> SelResult<Vec<ValidatedAction>> {
        let actions_array = parsed
            .get("actions")
            .and_then(|v| v.as_array())
            .ok_or_else(|| {
                SovereignError::InvalidMissionFormat(
                    "Missing or invalid 'actions' array".to_string(),
                )
            })?;

        let mut actions = Vec::with_capacity(actions_array.len());

        for (i, action_val) in actions_array.iter().enumerate() {
            let cmd = action_val
                .get("command")
                .and_then(|v| v.as_str())
                .ok_or_else(|| {
                    SovereignError::InvalidMissionFormat(format!("Action {}: missing 'command'", i))
                })?;

            let args = action_val
                .get("args")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str())
                        .map(|s| s.to_string())
                        .collect()
                })
                .unwrap_or_default();

            actions.push(ValidatedAction {
                command: cmd.to_string(),
                args,
            });
        }

        Ok(actions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hmac_validation() {
        let config = ValidationConfig::default();
        let validator = Validator::new(config);

        let mission = r#"{"actions":[{"command":"echo","args":["test"]}]}"#;
        let result = validator.validate(mission);

        assert!(result.is_ok());
        let validated = result.unwrap();
        assert!(!validated.validation_proof_str().is_empty());
    }

    #[test]
    fn test_reject_cat() {
        let config = ValidationConfig::default();
        let validator = Validator::new(config);

        let mission = r#"{"actions":[{"command":"cat","args":["file.txt"]}]}"#;
        let result = validator.validate(mission);

        assert!(result.is_err());
        match result {
            Err(SovereignError::CapabilityViolation(msg)) => {
                assert!(msg.contains("cat"));
            }
            _ => panic!("Expected CapabilityViolation"),
        }
    }

    #[test]
    fn test_reject_path_traversal() {
        let config = ValidationConfig::default();
        let validator = Validator::new(config);

        let mission = r#"{"actions":[{"command":"echo","args":["../../../etc/passwd"]}]}"#;
        let result = validator.validate(mission);

        assert!(result.is_err());
        match result {
            Err(SovereignError::WorkspaceViolation(_)) => {}
            _ => panic!("Expected WorkspaceViolation"),
        }
    }
}
