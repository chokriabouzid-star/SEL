use std::fs;
use std::env;
use std::process;
use sel_validator::Validator;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_help();
        process::exit(1);
    }
    
    let command = &args[1];
    
    match command.as_str() {
        "validate" | "check" | "--validate" | "--check" => {
            if args.len() < 3 {
                eprintln!("❌ Please provide a mission file");
                print_help();
                process::exit(1);
            }
            if let Err(e) = validate_mission(&args[2]) {
                eprintln!("❌ Error: {}", e);
                process::exit(1);
            }
        }
        "rules" | "--rules" => {
            print_rules();
        }
        "test" | "--test" => {
            if let Err(e) = run_test() {
                eprintln!("❌ Test failed: {}", e);
                process::exit(1);
            }
        }
        "help" | "-h" | "--help" => {
            print_help();
        }
        _ => {
            // إذا كان الملف موجوداً، تحقق منه مباشرة
            if fs::metadata(command).is_ok() {
                if let Err(e) = validate_mission(command) {
                    eprintln!("❌ Error: {}", e);
                    process::exit(1);
                }
            } else {
                eprintln!("❌ Unknown command or file not found: {}", command);
                print_help();
                process::exit(1);
            }
        }
    }
}

fn print_help() {
    println!("SEL Validator v0.1.0 - Mechanical Pattern Matcher");
    println!("Usage:");
    println!("  sel-validate <mission.json>           Validate a mission file");
    println!("  sel-validate validate <mission.json>  Validate a mission file");
    println!("  sel-validate rules                    List all validation rules");
    println!("  sel-validate test                     Run a test mission");
    println!("  sel-validate help                     Show this help");
}

fn print_rules() {
    println!("SEL Validator Rules (v0.1.0 - 18 rules)");
    println!("======================================");
    println!("\nExistence Rules (8):");
    println!("  1. MISSION_HAS_ID");
    println!("  2. MISSION_HAS_VERSION");
    println!("  3. MISSION_HAS_EXECUTION");
    println!("  4. EXECUTION_HAS_ACTIONS");
    println!("  5. ACTIONS_NON_EMPTY");
    println!("  6. EACH_ACTION_HAS_ID");
    println!("  7. EACH_ACTION_HAS_COMMAND");
    println!("  8. EACH_ACTION_HAS_VERIFICATION");
    
    println!("\nPattern Rules (6):");
    println!("  9. ID_MATCHES_REGEX");
    println!("  10. VERSION_MATCHES_SEMVER");
    println!("  11. ACTION_TYPE_IS_COMMAND");
    println!("  12. WORKING_DIRECTORY_CONTAINS_WORKSPACE_VAR");
    println!("  13. TIMEOUT_BETWEEN_1_AND_3600");
    println!("  14. COMMAND_LENGTH_UNDER_1000_CHARS");
    
    println!("\nSafety Rules (4):");
    println!("  15. NO_FORBIDDEN_COMMANDS");
    println!("  16. NO_NETWORK_FLAGS");
    println!("  17. NO_SYSTEM_PATHS");
    println!("  18. NO_SHELL_CONTROL_CHARS");
}

fn validate_mission(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔬 SEL Validator v0.1.0");
    println!("📄 Validating: {}", file_path);
    
    let content = fs::read_to_string(file_path)?;
    let validator = Validator::new();
    
    match validator.validate_json(&content) {
        Ok(result) => {
            match result.verdict {
                sel_validator::Verdict::Valid => {
                    println!("✅ Mission is VALID");
                    println!("   Rules applied: {}", result.rules_applied);
                    println!("   Rules passed: {}", result.rules_passed);
                }
                sel_validator::Verdict::Invalid => {
                    println!("❌ Mission is INVALID");
                    println!("   Rules applied: {}", result.rules_applied);
                    println!("   Rules passed: {}", result.rules_passed);
                    println!("   Violations found: {}", result.violations.len());
                    
                    if !result.violations.is_empty() {
                        println!("\n📋 Violations:");
                        for (i, violation) in result.violations.iter().enumerate() {
                            println!("   {}. {}: {}", i + 1, violation.rule, violation.fact);
                            println!("      Location: {}", violation.location);
                        }
                    }
                }
            }
        }
        Err(e) => {
            return Err(format!("Failed to parse mission: {}", e).into());
        }
    }
    
    Ok(())
}

fn run_test() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧪 Running constitutional test...");
    
    // مهمة اختبارية
    let test_mission = r#"{
        "id": "test-build",
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
    }"#;
    
    let validator = Validator::new();
    match validator.validate_json(test_mission) {
        Ok(result) => {
            if result.verdict == sel_validator::Verdict::Valid {
                println!("✅ Test mission is VALID");
                println!("   All {} rules passed", result.rules_passed);
            } else {
                println!("❌ Test mission is INVALID");
                for violation in result.violations {
                    println!("   - {}", violation.rule);
                }
            }
        }
        Err(e) => {
            return Err(format!("Test failed: {}", e).into());
        }
    }
    
    Ok(())
}
