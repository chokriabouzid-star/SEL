//! # Resource Exhaustion Tests
//! 🔒 DETERMINISTIC - No randomness

use sel_common::{ResourceKind, SovereignError};
use sel_engine::{MissionExecutor, ResourceLimits, WorkspaceMode};
use sel_validator::{ValidationConfig, Validator};

#[test]
fn test_tick_limit_enforced() -> Result<(), SovereignError> {
    println!("\n📏 RESOURCE TEST: Max Ticks (10,000)");

    let action_count = 5001;
    let mut actions = Vec::new();
    for i in 0..action_count {
        actions.push(format!(r#"{{"command": "echo", "args": ["{}"]}}"#, i));
    }

    let config = ValidationConfig {
        max_actions: action_count + 10,
        ..Default::default()
    };

    let mission_json = format!(r#"{{"actions": [{}]}}"#, actions.join(","));

    let validator = Validator::new(config);
    let validated = validator.validate(&mission_json)?;
    let mission_hash = validated.mission_hash();

    let custom_limits = ResourceLimits::new(action_count + 10, 10_000, 1_048_576, 102_400, 10_000);

    let mut executor =
        MissionExecutor::new_with_limits(WorkspaceMode::ReadOnly, &mission_hash, custom_limits)?;

    let result = executor.execute(validated);

    // Workspace no longer auto-cleans on Drop (F-002 fix).
    executor.workspace.cleanup().ok();

    match result {
        Err(SovereignError::ResourceExhaustion {
            kind,
            limit,
            requested,
        }) => {
            assert_eq!(kind, ResourceKind::Ticks);
            assert_eq!(limit, 10_000);
            assert!(requested > 10_000);
            println!(
                "  ✅ Blocked ticks (limit={}, requested={})",
                limit, requested
            );
        }
        _ => panic!("❌ Should have exceeded tick limit"),
    }

    Ok(())
}

#[test]
fn test_max_facts_limit_enforced() {
    use sel_engine::engine::{ResourceLimits, WorkspaceMode};
    use sel_validator::{ValidationConfig, Validator};

    let config = ValidationConfig::default();
    let validator = Validator::new(config);
    let mission_json = r#"{"actions":[{"command":"echo","args":["hi"]}]}"#;
    let validated = validator.validate(mission_json).unwrap();

    let mut limits = ResourceLimits::core_compliant();
    limits.max_facts = 3;

    let mut executor = sel_engine::engine::MissionExecutor::new_with_limits(
        WorkspaceMode::ReadOnly,
        &validated.mission_hash(),
        limits,
    )
    .unwrap();

    let result = executor.execute(validated);

    // Workspace no longer auto-cleans on Drop (F-002 fix).
    executor.workspace.cleanup().ok();

    assert!(result.is_err(), "يجب أن يفشل عند تجاوز max_facts");

    match result {
        Err(sel_common::SovereignError::ResourceExhaustion { kind, limit, .. }) => {
            assert_eq!(kind, sel_common::ResourceKind::Facts);
            assert_eq!(limit, 3);
        }
        other => panic!("خطأ غير متوقع: {:?}", other),
    }
}
