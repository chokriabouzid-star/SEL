//! Day 4 Integration Tests

use sel_validator::{Validator, ValidationResult};
use serde_json::json;

#[test]
fn test_end_to_end_validation() {
    let mut validator = Validator::new();
    
    let mission = json!({
        "name": "integration-test",
        "actions": [
            {"type": "command", "command": "echo", "args": ["hello"]}
        ]
    });
    
    match validator.validate(&mission) {
        ValidationResult::Valid(validated) => {
            assert!(validated.validation_proof().starts_with("hmac-sha256:"));
            assert!(validated.verify_proof().is_ok());
        }
        ValidationResult::Invalid { .. } => {
            panic!("Should be valid");
        }
    }
}

#[test]
fn test_multiple_validations_same_mission() {
    let mut validator = Validator::new();
    
    let mission = json!({
        "name": "cache-test",
        "actions": []
    });
    
    // First validation
    let result1 = validator.validate(&mission);
    
    // Second validation (should hit cache)
    let result2 = validator.validate(&mission);
    
    match (result1, result2) {
        (ValidationResult::Valid(v1), ValidationResult::Valid(v2)) => {
            // Proofs should be identical (same mission)
            assert_eq!(v1.validation_proof(), v2.validation_proof());
        }
        _ => panic!("Both should be valid"),
    }
}

#[test]
fn test_workspace_mode_detection() {
    let mut validator = Validator::new();
    
    // Read-only mission
    let readonly = json!({
        "name": "readonly",
        "actions": [
            {"type": "command", "command": "echo", "args": ["test"]}
        ]
    });
    
    // Read-write mission
    let readwrite = json!({
        "name": "readwrite",
        "actions": [
            {"type": "file_write", "path": "output.txt", "content": "test"}
        ]
    });
    
    match validator.validate(&readonly) {
        ValidationResult::Valid(v) => {
            assert_eq!(v.workspace_mode(), sel_validator::WorkspaceMode::ReadOnly);
        }
        _ => panic!("Should be valid"),
    }
    
    match validator.validate(&readwrite) {
        ValidationResult::Valid(v) => {
            assert_eq!(v.workspace_mode(), sel_validator::WorkspaceMode::ReadWrite);
        }
        _ => panic!("Should be valid"),
    }
}
