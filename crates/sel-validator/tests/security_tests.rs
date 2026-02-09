//! Security tests for SEL Validator

use sel_validator::{Validator, ValidationResult};
use serde_json::json;

#[test]
fn test_path_traversal_detection() {
    let mut validator = Validator::new();
    
    // Test ../ traversal
    let mission = json!({
        "name": "path-traversal-test",
        "actions": [
            {"type": "command", "command": "cat", "args": ["../../etc/passwd"]}
        ]
    });
    
    let result = validator.validate(&mission);
    assert!(matches!(result, ValidationResult::Invalid { .. }));
    
    if let ValidationResult::Invalid { errors, .. } = result {
        let has_path_error = errors.iter().any(|e| e.error_type.to_string() == "PathEscape");
        assert!(has_path_error, "Should detect path traversal");
    }
}

#[test]
fn test_forbidden_commands() {
    let mut validator = Validator::new();
    
    // Test rm command
    let mission = json!({
        "name": "forbidden-test",
        "actions": [
            {"type": "command", "command": "rm", "args": ["-rf", "/"]}
        ]
    });
    
    let result = validator.validate(&mission);
    assert!(matches!(result, ValidationResult::Invalid { .. }));
}

#[test]
fn test_valid_mission() {
    let mut validator = Validator::new();
    
    // Test valid mission
    let mission = json!({
        "name": "valid-test",
        "actions": [
            {"type": "command", "command": "echo", "args": ["hello"]},
            {"type": "command", "command": "ls", "args": ["-la", "/tmp"]}
        ]
    });
    
    let result = validator.validate(&mission);
    assert!(matches!(result, ValidationResult::Valid(_)));
}

#[test]
fn test_dangerous_patterns() {
    let mut validator = Validator::new();
    
    let dangerous_args = [
        "../../etc/passwd",
        "/bin/bash",
        "/root/.ssh/id_rsa",
        "| rm -rf /",
        "; shutdown now",
        "$(rm -rf /)",
        "`reboot`",
        "& exit",
    ];
    
    for arg in dangerous_args.iter() {
        let mission = json!({
            "name": "dangerous-test",
            "actions": [
                {"type": "command", "command": "echo", "args": [arg]}
            ]
        });
        
        let result = validator.validate(&mission);
        assert!(
            matches!(result, ValidationResult::Invalid { .. }),
            "Should reject dangerous argument: {}",
            arg
        );
    }
}
