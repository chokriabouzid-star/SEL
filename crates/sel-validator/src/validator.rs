//! # Sovereign Validator Implementation
//! SEL Extended 1.1 - Dual Crypto Support (HMAC + Ed25519)

use serde_json::{Value, from_str};
use sel_common::{SovereignError, SelResult, canonicalize_json, ResourceKind};
use sha2::{Sha256, Digest};

use crate::{
    types::{
        ValidatedMission,
        ExecutionCapabilities,
        ValidatedAction,
        ValidationProof,
    },
    crypto_authority::{CryptoAuthority, SignatureType},
    rules::{validate_security_rules, SecurityViolation},
};

const CORE_ALLOWED_COMMANDS: [&str; 2] = ["echo", "pwd"];

#[derive(Debug, Clone)]
pub struct ValidationConfig {
    pub max_actions: usize,
    pub strict_mode: bool,
    /// Which crypto to use for validation proofs
    pub signature_type: SignatureType,
}

impl Default for ValidationConfig {
    fn default() -> Self {
        Self {
            max_actions: 1000,
            strict_mode: true,
            signature_type: SignatureType::Hmac,  // Core 1.0 compatibility
        }
    }
}

pub struct Validator {
    config: ValidationConfig,
    crypto: CryptoAuthority,
}

impl Validator {
    pub fn new(config: ValidationConfig) -> Self {
        let crypto = match config.signature_type {
            SignatureType::Hmac => CryptoAuthority::new_hmac(),
            #[cfg(feature = "ed25519")]
            SignatureType::Ed25519 => CryptoAuthority::new_ed25519(),
        };
        
        Self {
            config,
            crypto,
        }
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
        
        // Command whitelist
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
        
        // Generate cryptographic proof with selected algorithm
        let signature = self.crypto.sign(&canonical);
        let proof = ValidationProof::new(signature, self.config.signature_type);
        
        // Create ValidatedMission
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
    
    /// Get the public key (if using Ed25519)
    #[cfg(feature = "ed25519")]
    pub fn public_key(&self) -> Option<String> {
        self.crypto.public_key()
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
        assert_eq!(validated.signature_type(), SignatureType::Hmac);
        assert!(!validated.validation_proof_str().is_empty());
    }
    
    #[cfg(feature = "ed25519")]
    #[test]
    fn test_ed25519_validation() {
        let mut config = ValidationConfig::default();
        config.signature_type = SignatureType::Ed25519;
        
        let validator = Validator::new(config);
        let mission = r#"{"actions":[{"command":"echo","args":["test"]}]}"#;
        let result = validator.validate(mission);
        
        assert!(result.is_ok());
        let validated = result.unwrap();
        assert_eq!(validated.signature_type(), SignatureType::Ed25519);
        assert!(!validated.validation_proof_str().is_empty());
        
        // Public key should be available
        assert!(validator.public_key().is_some());
    }
}
