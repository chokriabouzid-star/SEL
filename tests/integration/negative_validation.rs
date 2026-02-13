//! # Negative Validation Tests
//! SEL Core 1.0 - يجب رفض أي أمر غير echo/pwd

use sel_validator::{Validator, ValidationConfig};
use sel_common::SovereignError;

#[test]
fn test_reject_ls_command() {
    println!("\n🚫 NEGATIVE TEST: Reject 'ls' command");
    println!("=====================================");
    
    let mission_json = r#"{
        "actions": [
            {
                "type": "builtin",
                "command": "ls",
                "args": ["-la"]
            }
        ]
    }"#;
    
    let validator = Validator::new(ValidationConfig::default());
    let result = validator.validate(mission_json);
    
    match result {
        Err(SovereignError::CapabilityViolation(msg)) => {
            println!("  ✅ REJECTED: {}", msg);
            assert!(msg.contains("ls"));
        }
        Ok(_) => panic!("❌ 'ls' command should be rejected"),
        Err(e) => panic!("❌ Wrong error type: {}", e),
    }
    
    println!("=====================================\n");
}

#[test]
fn test_reject_cat_command() {
    println!("\n🚫 NEGATIVE TEST: Reject 'cat' command");
    println!("=====================================");
    
    let mission_json = r#"{
        "actions": [
            {
                "type": "builtin",
                "command": "cat",
                "args": ["/etc/passwd"]
            }
        ]
    }"#;
    
    let validator = Validator::new(ValidationConfig::default());
    let result = validator.validate(mission_json);
    
    match result {
        Err(SovereignError::CapabilityViolation(msg)) => {
            println!("  ✅ REJECTED: {}", msg);
            assert!(msg.contains("cat"));
        }
        Ok(_) => panic!("❌ 'cat' command should be rejected"),
        Err(e) => panic!("❌ Wrong error type: {}", e),
    }
    
    println!("=====================================\n");
}

#[test]
fn test_reject_path_traversal() {
    println!("\n🚫 NEGATIVE TEST: Reject path traversal");
    println!("=====================================");
    
    let mission_json = r#"{
        "actions": [
            {
                "type": "builtin",
                "command": "echo",
                "args": ["../../../etc/shadow"]
            }
        ]
    }"#;
    
    let validator = Validator::new(ValidationConfig::default());
    let result = validator.validate(mission_json);
    
    match result {
        Err(SovereignError::WorkspaceViolation(path)) => {
            println!("  ✅ REJECTED: Path traversal detected: {}", path);
            assert!(path.contains(".."));
        }
        Ok(_) => panic!("❌ Path traversal should be rejected"),
        Err(e) => panic!("❌ Wrong error type: {}", e),
    }
    
    println!("=====================================\n");
}

#[test]
fn test_accept_echo_pwd_only() {
    println!("\n✅ POSITIVE TEST: Accept echo/pwd only");
    println!("=====================================");
    
    let mission_json = r#"{
        "actions": [
            {
                "type": "builtin",
                "command": "echo",
                "args": ["hello"]
            },
            {
                "type": "builtin",
                "command": "pwd",
                "args": []
            }
        ]
    }"#;
    
    let validator = Validator::new(ValidationConfig::default());
    let result = validator.validate(mission_json);
    
    assert!(result.is_ok(), "✅ echo/pwd should be accepted");
    println!("  ✅ ACCEPTED: echo and pwd are allowed in Core 1.0");
    println!("=====================================\n");
}
