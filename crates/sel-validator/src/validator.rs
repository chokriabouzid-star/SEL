//! Sovereign Validator - The Gatekeeper

use std::collections::HashSet;
use std::time::Duration;
use serde_json::Value;
use crate::types::*;

/// Validation error
#[derive(Debug, Clone)]
pub struct ValidationError {
    pub error_type: ErrorType,
    pub message: String,
    pub location: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorType {
    SchemaViolation,
    ForbiddenCommand,
    PathEscape,
    InvalidCapability,
}

impl std::fmt::Display for ErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorType::SchemaViolation => write!(f, "SchemaViolation"),
            ErrorType::ForbiddenCommand => write!(f, "ForbiddenCommand"),
            ErrorType::PathEscape => write!(f, "PathEscape"),
            ErrorType::InvalidCapability => write!(f, "InvalidCapability"),
        }
    }
}

/// Validation result
#[derive(Debug)]
pub enum ValidationResult {
    Valid(ValidatedMission),
    Invalid {
        errors: Vec<ValidationError>,
        suggestions: Vec<String>,
    },
}

/// Sovereign Validator
pub struct Validator {
    forbidden_commands: HashSet<String>,
    _allowed_paths: HashSet<String>,
    capabilities: ExecutionCapabilities,
}

impl Validator {
    pub fn new() -> Self {
        let mut forbidden = HashSet::new();
        forbidden.insert("rm".to_string());
        forbidden.insert("dd".to_string());
        forbidden.insert("mkfs".to_string());
        forbidden.insert("format".to_string());
        forbidden.insert("shutdown".to_string());
        forbidden.insert("halt".to_string());
        forbidden.insert("poweroff".to_string());
        
        let mut allowed_paths = HashSet::new();
        allowed_paths.insert("/tmp".to_string());
        allowed_paths.insert("/home".to_string());
        allowed_paths.insert("/var/tmp".to_string());
        
        Self {
            forbidden_commands: forbidden,
            _allowed_paths: allowed_paths,
            capabilities: ExecutionCapabilities {
                allowed_commands: vec!["echo".to_string(), "ls".to_string(), "cat".to_string()],
                allowed_paths: vec!["/tmp".to_string(), "/home".to_string()],
                max_execution_time: Duration::from_secs(30),
                workspace_mode: WorkspaceMode::Strict,
            },
        }
    }
    
    pub fn validate(&mut self, mission: &Value) -> ValidationResult {
        let mut errors = Vec::new();
        let mut suggestions = Vec::new();
        
        // Validate mission structure
        if !mission.is_object() {
            errors.push(ValidationError {
                error_type: ErrorType::SchemaViolation,
                message: "Mission must be a JSON object".to_string(),
                location: None,
            });
            return ValidationResult::Invalid { errors, suggestions };
        }
        
        // Validate name field
        if let Some(name) = mission.get("name") {
            if !name.is_string() {
                errors.push(ValidationError {
                    error_type: ErrorType::SchemaViolation,
                    message: "Mission name must be a string".to_string(),
                    location: Some("name".to_string()),
                });
            }
        } else {
            errors.push(ValidationError {
                error_type: ErrorType::SchemaViolation,
                message: "Mission must have a 'name' field".to_string(),
                location: None,
            });
        }
        
        // Validate actions
        if let Some(actions) = mission.get("actions") {
            if !actions.is_array() {
                errors.push(ValidationError {
                    error_type: ErrorType::SchemaViolation,
                    message: "Actions must be an array".to_string(),
                    location: Some("actions".to_string()),
                });
            } else if let Some(arr) = actions.as_array() {
                for (i, action) in arr.iter().enumerate() {
                    self.validate_action(action, i, &mut errors);
                }
            }
        } else {
            errors.push(ValidationError {
                error_type: ErrorType::SchemaViolation,
                message: "Mission must have 'actions' field".to_string(),
                location: None,
            });
        }
        
        // If errors, return invalid
        if !errors.is_empty() {
            suggestions.push("Check mission structure against schema".to_string());
            suggestions.push("Use only allowed commands".to_string());
            suggestions.push("Avoid path traversal attempts".to_string());
            return ValidationResult::Invalid { errors, suggestions };
        }
        
        // Generate validation proof
        let proof = match self.generate_proof(mission) {
            Ok(p) => p,
            Err(e) => {
                errors.push(ValidationError {
                    error_type: ErrorType::SchemaViolation,
                    message: format!("Failed to generate validation proof: {}", e),
                    location: None,
                });
                return ValidationResult::Invalid { errors, suggestions };
            }
        };
        
        // Create validated mission
        let validated = ValidatedMission::new(
            self.capabilities.clone(),
            proof,
        );
        
        ValidationResult::Valid(validated)
    }
    
    fn validate_action(&self, action: &Value, index: usize, errors: &mut Vec<ValidationError>) {
        if !action.is_object() {
            errors.push(ValidationError {
                error_type: ErrorType::SchemaViolation,
                message: format!("Action {} must be an object", index),
                location: Some(format!("actions[{}]", index)),
            });
            return;
        }
        
        // Check action type
        if let Some(action_type) = action.get("type") {
            if let Some(type_str) = action_type.as_str() {
                match type_str {
                    "command" => self.validate_command_action(action, index, errors),
                    _ => {
                        errors.push(ValidationError {
                            error_type: ErrorType::SchemaViolation,
                            message: format!("Unknown action type: {}", type_str),
                            location: Some(format!("actions[{}].type", index)),
                        });
                    }
                }
            } else {
                errors.push(ValidationError {
                    error_type: ErrorType::SchemaViolation,
                    message: "Action type must be a string".to_string(),
                    location: Some(format!("actions[{}].type", index)),
                });
            }
        } else {
            errors.push(ValidationError {
                error_type: ErrorType::SchemaViolation,
                message: "Action must have 'type' field".to_string(),
                location: Some(format!("actions[{}]", index)),
            });
        }
    }
    
    fn validate_command_action(&self, action: &Value, index: usize, errors: &mut Vec<ValidationError>) {
        // Check command field
        if let Some(command) = action.get("command") {
            if let Some(cmd_str) = command.as_str() {
                if self.forbidden_commands.contains(cmd_str) {
                    errors.push(ValidationError {
                        error_type: ErrorType::ForbiddenCommand,
                        message: format!("Forbidden command: {}", cmd_str),
                        location: Some(format!("actions[{}].command", index)),
                    });
                }
            } else {
                errors.push(ValidationError {
                    error_type: ErrorType::SchemaViolation,
                    message: "Command must be a string".to_string(),
                    location: Some(format!("actions[{}].command", index)),
                });
            }
        } else {
            errors.push(ValidationError {
                error_type: ErrorType::SchemaViolation,
                message: "Command action must have 'command' field".to_string(),
                location: Some(format!("actions[{}]", index)),
            });
        }
        
        // Check args (optional) - Add path traversal detection
        if let Some(args) = action.get("args") {
            if !args.is_array() {
                errors.push(ValidationError {
                    error_type: ErrorType::SchemaViolation,
                    message: "Args must be an array".to_string(),
                    location: Some(format!("actions[{}].args", index)),
                });
            } else if let Some(arr) = args.as_array() {
                for (arg_idx, arg) in arr.iter().enumerate() {
                    if let Some(arg_str) = arg.as_str() {
                        self.detect_path_traversal(arg_str, index, arg_idx, errors);
                    }
                }
            }
        }
    }
    
    fn detect_path_traversal(&self, arg: &str, action_idx: usize, arg_idx: usize, errors: &mut Vec<ValidationError>) {
        // Detect common path traversal patterns
        let dangerous_patterns = [
            "../",          // Directory traversal
            "..\\",         // Windows directory traversal
            "/etc/",        // System directories
            "/bin/",
            "/sbin/",
            "/usr/bin/",
            "/root/",
            "/var/log/",
            "~/.ssh/",      // SSH keys
            "~/.bashrc",    // Shell configs
            "*",            // Wildcards
            "?",            // Wildcards
            "|",            // Pipe
            "&",            // Background process
            ";",            // Command separator
            "`",            // Command substitution
            "$(",           // Command substitution
        ];
        
        for pattern in dangerous_patterns.iter() {
            if arg.contains(pattern) {
                errors.push(ValidationError {
                    error_type: ErrorType::PathEscape,
                    message: format!("Potential path traversal detected: '{}' in argument", pattern),
                    location: Some(format!("actions[{}].args[{}]", action_idx, arg_idx)),
                });
                break;
            }
        }
        
        // Detect absolute paths outside allowed directories
        if arg.starts_with('/') {
            let mut allowed = false;
            for allowed_path in &self.capabilities.allowed_paths {
                if arg.starts_with(allowed_path) {
                    allowed = true;
                    break;
                }
            }
            
            if !allowed {
                errors.push(ValidationError {
                    error_type: ErrorType::PathEscape,
                    message: format!("Access to path '{}' not allowed", arg),
                    location: Some(format!("actions[{}].args[{}]", action_idx, arg_idx)),
                });
            }
        }
    }
    
    fn generate_proof(&self, mission: &Value) -> Result<String, ProofError> {
        use sha2::{Sha256, Digest};
        
        // Serialize mission
        let serialized = serde_json::to_vec(mission)
            .map_err(|e| ProofError::SerializationFailed(e.to_string()))?;
        
        // Create hash
        let mut hasher = Sha256::new();
        hasher.update(&serialized);
        hasher.update(self.capabilities.max_execution_time.as_secs().to_be_bytes());
        let result = hasher.finalize();
        
        // Format as hex
        let proof = format!("hmac-sha256:{}", hex::encode(result));
        
        Ok(proof)
    }
}

#[cfg(test)]
mod additional_tests {
    use super::*;

    #[test]
    fn test_validator_cache() {
        let mut cache = ValidatorCache::new(2);
        
        let v1 = VALIDATOR_VERSION.clone();
        let mission = ValidatedMission {
            raw: serde_json::json!({"name": "test"}),
            validation_proof: "proof1".to_string(),
            validation_timestamp: chrono::Utc::now(),
            validator_version: v1.clone(),
            capabilities: ExecutionCapabilities::default(),
            workspace_mode: WorkspaceMode::ReadOnly,
        };
        
        cache.insert("hash1", &v1, mission.clone());
        assert!(cache.get("hash1", &v1).is_some());
        assert!(cache.get("hash2", &v1).is_none());
    }

    #[test]
    fn test_path_allowlist() {
        let validator = Validator::new();
        
        assert!(validator.path_is_allowed("/tmp/test"));
        assert!(!validator.path_is_allowed("/etc/passwd"));
    }

    #[test]
    fn test_binary_detection() {
        assert!(!is_binary_command("echo"));
        assert!(!is_binary_command("pwd"));
        assert!(is_binary_command("gcc"));
        assert!(is_binary_command("python"));
    }
}
