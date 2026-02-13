//! # SEL Engine - Core 1.0 Demonstration
//! 🔒 NO RANDOMNESS - Fully Deterministic

use sel_engine::{MissionExecutor, WorkspaceMode};
use sel_validator::{Validator, ValidationConfig};
use sel_common::SovereignError;

fn main() -> Result<(), SovereignError> {
    println!("🔐 SEL Engine Core 1.0 - DETERMINISTIC");
    println!("=======================================");
    
    let mission_json = r#"{
        "name": "sel-core-1.0-demo",
        "actions": [
            {"command": "echo", "args": ["Hello", "from", "SEL", "Core", "1.0!"]},
            {"command": "pwd", "args": []}
        ]
    }"#;
    
    // ✅ لا استخدام لـ ? هنا - Validator::new لا يرجع Result
    let validator = Validator::new(ValidationConfig::default());
    let validated = validator.validate(mission_json)?;
    let mission_hash = validated.mission_hash();
    
    let mut executor = MissionExecutor::new(
        WorkspaceMode::ReadOnly, 
        &mission_hash
    )?;
    
    let report = executor.execute(validated)?;
    
    println!("\n✅ Execution Complete");
    println!("   • Workspace: {}", executor.workspace.uuid());
    println!("   • Hash: {}", &report.final_hash[..16]);
    println!("   • Ticks: {}", report.logical_ticks);
    
    Ok(())
}
