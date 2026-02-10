use sel_validator::{Validator, ValidationResult, ErrorType};
use serde_json::json;

#[test]
fn test_valid_mission() {
    let mut validator = Validator::new();
    let mission = json!({
        "name": "valid-mission",
        "actions": [
            {"type": "command", "command": "echo", "args": ["hello"]}
        ]
    });

    match validator.validate(&mission) {
        ValidationResult::Valid(_) => {},
        ValidationResult::Invalid { .. } => panic!("Should be valid"),
    }
}

#[test]
fn test_path_traversal_detection() {
    let mut validator = Validator::new();
    
    // Test 1: Path traversal in command args
    let mission = json!({
        "name": "path-traversal",
        "actions": [
            {"type": "command", "command": "cat", "args": ["../../etc/passwd"]}
        ]
    });

    match validator.validate(&mission) {
        ValidationResult::Valid(_) => panic!("Should be invalid"),
        ValidationResult::Invalid { errors, .. } => {
            let has_path_error = errors.iter().any(|e| e.error_type == ErrorType::PathEscape);
            assert!(has_path_error, "Should detect path traversal in args");
        }
    }
    
    // Test 2: Dangerous pattern
    let mission2 = json!({
        "name": "dangerous-pattern",
        "actions": [
            {"type": "command", "command": "echo", "args": ["test; rm -rf /"]}
        ]
    });
    
    match validator.validate(&mission2) {
        ValidationResult::Valid(_) => panic!("Should be invalid"),
        ValidationResult::Invalid { errors, .. } => {
            let has_path_error = errors.iter().any(|e| e.error_type == ErrorType::PathEscape);
            assert!(has_path_error, "Should detect dangerous pattern");
        }
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
