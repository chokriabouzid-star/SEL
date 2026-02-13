//! # Stress Determinism Test
//! 🔒 20 executions - MUST have identical hash

use sel_engine::{MissionExecutor, WorkspaceMode};
use sel_validator::{Validator, ValidationConfig};
use sel_common::SovereignError;

const EXECUTION_COUNT: usize = 20;

#[test]
fn test_20_executions_identical_hash() -> Result<(), SovereignError> {
    println!("\n🔬 STRESS DETERMINISM TEST - 20 EXECUTIONS");
    
    let mission_json = r#"{
        "actions": [
            {"command": "echo", "args": ["determinism", "test"]},
            {"command": "pwd", "args": []}
        ]
    }"#;
    
    // ✅ NO ? here - Validator::new doesn't return Result
    let validator = Validator::new(ValidationConfig::default());
    let validated = validator.validate(mission_json)?;
    let mission_hash = validated.mission_hash();
    
    let mut previous_hash = None;
    
    for i in 1..=EXECUTION_COUNT {
        let mut executor = MissionExecutor::new(
            WorkspaceMode::ReadOnly, 
            &mission_hash
        )?;
        let report = executor.execute(validated.clone())?;
        
        println!("  ⚙️  {:2}: Hash = {}", i, &report.final_hash[..16]);
        
        if let Some(ref prev) = previous_hash {
            assert_eq!(*prev, report.final_hash, "Hash mismatch at execution {}", i);
        }
        previous_hash = Some(report.final_hash);
    }
    
    println!("\n✅✅✅ DETERMINISM VERIFIED: {} executions", EXECUTION_COUNT);
    Ok(())
}
