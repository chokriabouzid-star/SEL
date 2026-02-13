//! # Resource Exhaustion Tests
//! 🔒 DETERMINISTIC - No randomness

use sel_engine::{MissionExecutor, WorkspaceMode, ResourceLimits};
use sel_validator::{Validator, ValidationConfig};
use sel_common::{SovereignError, ResourceKind};

#[test]
fn test_tick_limit_enforced() -> Result<(), SovereignError> {
    println!("\n📏 RESOURCE TEST: Max Ticks (10,000)");
    
    let action_count = 5001;
    let mut actions = Vec::new();
    for i in 0..action_count {
        actions.push(format!(
            r#"{{"command": "echo", "args": ["{}"]}}"#,
            i
        ));
    }

    let mut config = ValidationConfig::default();
    config.max_actions = action_count + 10;
    
    let mission_json = format!(
        r#"{{"actions": [{}]}}"#,
        actions.join(",")
    );

    // ✅ NO ? here - Validator::new doesn't return Result
    let validator = Validator::new(config);
    let validated = validator.validate(&mission_json)?;
    let mission_hash = validated.mission_hash();

    let custom_limits = ResourceLimits::new(
        action_count + 10,
        10_000,
        1_048_576,
        102_400,
        10_000,
    );
    
    let mut executor = MissionExecutor::new_with_limits(
        WorkspaceMode::ReadOnly,
        &mission_hash,
        custom_limits
    )?;

    let result = executor.execute(validated);
    
    match result {
        Err(SovereignError::ResourceExhaustion { kind, limit, requested }) => {
            assert_eq!(kind, ResourceKind::Ticks);
            assert_eq!(limit, 10_000);
            assert!(requested > 10_000);
            println!("  ✅ Blocked ticks (limit={}, requested={})", limit, requested);
        }
        _ => panic!("❌ Should have exceeded tick limit"),
    }
    
    Ok(())
}
