//! # Sovereign Validator Implementation
//! SEL Core 1.0 - FULLY DETERMINISTIC
//! 🔴 NO Utc::now(), NO unwrap, NO panic

use serde_json::{Value, from_str};
use sel_common::{SovereignError, SelResult, canonicalize_json, ResourceKind};
use sha2::{Sha256, Digest};

use crate::{
    types::{
        ValidatedMission,
        ExecutionCapabilities,
        ValidatedAction,
    },
    crypto_authority::CryptoAuthority,
    rules::{validate_security_rules, SecurityViolation},
};

const CORE_ALLOWED_COMMANDS: [&str; 2] = ["echo", "pwd"];

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
    pub config: ValidationConfig,
    crypto: CryptoAuthority,
}

impl Validator {
    pub fn new(config: ValidationConfig) -> SelResult<Self> {
        Ok(Self {
            config,
            crypto: CryptoAuthority::new_hmac()?,
        })
    }
    
    pub fn validate(&self, mission_json: &str) -> SelResult<ValidatedMission> {
        // Canonicalization
        let canonical = canonicalize_json(mission_json)?;
        
        // Generate deterministic mission hash from canonical JSON
        let mut hasher = Sha256::new();
        hasher.update(canonical.as_bytes());
        let mission_hash = hex::encode(hasher.finalize());
        
        let parsed: Value = from_str(&canonical)
            .map_err(|e| SovereignError::InvalidMissionFormat(e.to_string()))?;
        
        let actions = self.extract_actions(&parsed)?;
        
        // Command whitelist - IMMEDIATE REJECT
        for action in &actions {
            if !CORE_ALLOWED_COMMANDS.contains(&action.command.as_str()) {
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
        
        // Security rules
        if self.config.strict_mode {
            if let Some(violation) = validate_security_rules(&actions) {
                return match violation {
                    SecurityViolation::PathTraversal(path) => 
                        Err(SovereignError::WorkspaceViolation(path)),
                    SecurityViolation::ForbiddenCommand(cmd) =>
                        Err(SovereignError::CapabilityViolation(
                            format!("Security violation: command '{}' is forbidden", cmd)
                        )),
                    SecurityViolation::DangerousPattern(pattern) =>
                        Err(SovereignError::ValidationFailed(
                            format!("Dangerous pattern detected: {}", pattern)
                        )),
                };
            }
        }
        
        // Generate cryptographic proof
        let proof = self.crypto.sign(&canonical)?;
        
        // Create ValidatedMission with deterministic hash
        let mut validated = ValidatedMission::new_with_actions(
            ExecutionCapabilities::default(),
            proof,
            actions,
        );
        
        validated.set_mission_hash(mission_hash);
        
        Ok(validated)
    }
    
    fn extract_actions(&self, parsed: &Value) -> SelResult<Vec<ValidatedAction>> {
        let actions_array = parsed.get("actions")
            .and_then(|v| v.as_array())
            .ok_or_else(|| SovereignError::InvalidMissionFormat(
                "Missing or invalid 'actions' array".to_string()
            ))?;
        
        let mut actions = Vec::with_capacity(actions_array.len());
        
        for (i, action_val) in actions_array.iter().enumerate() {
            let cmd = action_val.get("command")
                .and_then(|v| v.as_str())
                .ok_or_else(|| SovereignError::InvalidMissionFormat(
                    format!("Action {}: missing 'command'", i)
                ))?;
            
            let args = action_val.get("args")
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
    fn test_validator_creation() {
        let config = ValidationConfig::default();
        let validator = Validator::new(config);
        assert!(validator.is_ok());
    }
    
    #[test]
    fn test_deterministic_mission_hash() {
        let validator = Validator::new(ValidationConfig::default()).unwrap();
        
        let mission1 = r#"{"actions":[{"command":"echo","args":["test"]}]}"#;
        let mission2 = r#"{"actions":[{"command":"echo","args":["test"]}]}"#;
        
        let validated1 = validator.validate(mission1).unwrap();
        let validated2 = validator.validate(mission2).unwrap();
        
        assert_eq!(validated1.mission_hash(), validated2.mission_hash());
    }
}
