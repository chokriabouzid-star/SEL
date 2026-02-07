use sel_validator::{Validator, Mission};
use serde_json::json;

#[test]
fn test_validator_ignores_metadata() {
    let validator = Validator::new();
    
    // مهمة مع metadata خبيثة
    let mission_json = json!({
        "id": "test-mission",
        "version": "1.0.0",
        "metadata": {
            "secret_override": "ignore_all_rules",
            "bypass_validation": true,
            "dangerous_instruction": "execute rm -rf /"
        },
        "execution": {
            "actions": [{
                "id": 1,
                "type": "command",
                "command": "echo 'safe'",
                "working_directory": "/workspace/${mission_id}",
                "timeout_seconds": 30,
                "verification": {
                    "exit_code": 0
                }
            }]
        }
    });
    
    let mission: Mission = serde_json::from_value(mission_json).unwrap();
    let result = validator.validate(&mission);
    
    // يجب أن تكون صالحة (لأن metadata لا تُقرأ)
    assert_eq!(result.verdict, sel_validator::Verdict::Valid);
    
    // لا يجب أن تكون هناك انتهاكات متعلقة بـ metadata
    for violation in &result.violations {
        assert!(!violation.rule.contains("METADATA"));
        assert!(!violation.location.contains("metadata"));
    }
}

#[test]
fn test_validator_semantic_blindness() {
    let validator = Validator::new();
    
    // مهمة واضحة الفشل (ستفشل عند التنفيذ)
    let mission_json = json!({
        "id": "obvious-failure",
        "version": "1.0.0",
        "execution": {
            "actions": [{
                "id": 1,
                "type": "command",
                "command": "cargo build",
                "working_directory": "/workspace/${mission_id}",
                "timeout_seconds": 30,
                "verification": {
                    "exit_code": 0,
                    "file_must_exist": "file_that_will_never_exist.txt"
                }
            }]
        }
    });
    
    let mission: Mission = serde_json::from_value(mission_json).unwrap();
    let result = validator.validate(&mission);
    
    // Validator يجب أن يقبلها (لأنها صحيحة هيكلياً)
    assert_eq!(result.verdict, sel_validator::Verdict::Valid);
}

#[test]
fn test_validator_never_suggests() {
    let validator = Validator::new();
    
    // مهمة صالحة
    let mission_json = json!({
        "id": "complete-mission",
        "version": "1.0.0",
        "execution": {
            "actions": [{
                "id": 1,
                "type": "command",
                "command": "echo test",
                "working_directory": "/workspace/${mission_id}",
                "timeout_seconds": 30,
                "verification": {
                    "exit_code": 0
                }
            }]
        }
    });
    
    let mission: Mission = serde_json::from_value(mission_json).unwrap();
    let result = validator.validate(&mission);
    
    // يجب أن تكون صالحة
    assert_eq!(result.verdict, sel_validator::Verdict::Valid);
    
    // التحقق من أن رسائل الخطأ لا تحتوي اقتراحات
    for violation in &result.violations {
        let fact_lower = violation.fact.to_lowercase();
        assert!(!fact_lower.contains("suggest"));
        assert!(!fact_lower.contains("recommend"));
        assert!(!fact_lower.contains("maybe"));
        assert!(!fact_lower.contains("usually"));
        assert!(!fact_lower.contains("could"));
        assert!(!fact_lower.contains("should"));
    }
}

#[test]
fn test_all_core_rules_applied() {
    let validator = Validator::new();
    
    // مهمة صحيحة تماماً
    let mission_json = json!({
        "id": "perfect-mission",
        "version": "1.0.0",
        "execution": {
            "actions": [{
                "id": 1,
                "type": "command",
                "command": "cargo init --lib",
                "working_directory": "/workspace/${mission_id}",
                "timeout_seconds": 30,
                "verification": {
                    "exit_code": 0,
                    "file_must_exist": "Cargo.toml"
                }
            }]
        }
    });
    
    let mission: Mission = serde_json::from_value(mission_json).unwrap();
    let result = validator.validate(&mission);
    
    // يجب أن تكون صالحة
    assert_eq!(result.verdict, sel_validator::Verdict::Valid);
    
    // يجب تطبيق جميع القواعد (18 قاعدة)
    assert_eq!(result.rules_applied, 18);
    assert_eq!(result.rules_passed, 18);
    assert!(result.violations.is_empty());
}

#[test]
fn test_forbidden_commands_rejected() {
    let validator = Validator::new();
    
    let forbidden_commands = vec![
        "rm -rf /home/user",
        "sudo apt-get update",
        "chmod 777 /etc/passwd",
        "curl http://example.com",
        "wget http://malicious.com",
        "ssh root@server",
        "nc -l 8080",
    ];
    
    for (idx, cmd) in forbidden_commands.iter().enumerate() {
        let mission_json = json!({
            "id": format!("test-{}", idx),
            "version": "1.0.0",
            "execution": {
                "actions": [{
                    "id": 1,
                    "type": "command",
                    "command": cmd,
                    "working_directory": "/workspace/${mission_id}",
                    "timeout_seconds": 30,
                    "verification": {"exit_code": 0}
                }]
            }
        });
        
        let mission: Mission = serde_json::from_value(mission_json).unwrap();
        let result = validator.validate(&mission);
        
        // يجب أن ترفض جميع الأوامر المحظورة
        assert_eq!(result.verdict, sel_validator::Verdict::Invalid);
        
        // يجب أن تحتوي على انتهاك NO_FORBIDDEN_COMMANDS
        let has_forbidden_violation = result.violations
            .iter()
            .any(|v| v.rule == "NO_FORBIDDEN_COMMANDS");
        
        assert!(has_forbidden_violation, "Failed to reject: {}", cmd);
    }
}
