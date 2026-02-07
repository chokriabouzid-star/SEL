use sel_validator::Validator;

fn main() {
    let validator = Validator::new();
    
    // مهمة بسيطة للاختبار
    let test_json = r#"{
        "id": "simple-test",
        "version": "1.0.0",
        "execution": {
            "actions": [{
                "id": 1,
                "type": "command",
                "command": "echo hello",
                "working_directory": "/workspace/${mission_id}",
                "timeout_seconds": 30,
                "verification": {
                    "exit_code": 0
                }
            }]
        }
    }"#;
    
    match validator.validate_json(test_json) {
        Ok(result) => {
            println!("Result: {:?}", result.verdict);
            println!("Rules applied: {}", result.rules_applied);
            println!("Rules passed: {}", result.rules_passed);
            if !result.violations.is_empty() {
                println!("Violations:");
                for v in &result.violations {
                    println!("  - {}: {}", v.rule, v.fact);
                }
            }
        }
        Err(e) => println!("Error: {}", e),
    }
}
