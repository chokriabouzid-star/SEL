//! # SEL Engine - Core 1.0 Demonstration
//! 🔒 DETERMINISTIC - Prints all outputs

use sel_common::SovereignError;
use sel_engine::{MissionExecutor, WorkspaceMode};
use sel_validator::{ValidationConfig, Validator};
use std::fs;

fn main() -> Result<(), SovereignError> {
    println!("🔐 SEL Engine Core 1.0 - DETERMINISTIC");
    println!("=======================================");

    let mission_json = r#"{
        "name": "sel-intro",
        "actions": [
            {"command": "echo", "args": ["╔════════════════════════════════════════════╗"]},
            {"command": "echo", "args": ["║         SEL - Sovereign Execution Layer    ║"]},
            {"command": "echo", "args": ["╚════════════════════════════════════════════╝"]},
            {"command": "echo", "args": ["الإصدار: Core 1.0.0"]},
            {"command": "echo", "args": ["التاريخ: 2026-02-13"]},
            {"command": "echo", "args": ["الحالة: تجريبية - جاهزة للنشر"]},
            {"command": "echo", "args": ["الخصائص:", "  • تنفيذ حتمي 100%", "  • تواقيع HMAC", "  • حماية path traversal", "  • حدود الموارد"]},
            {"command": "echo", "args": ["اختبارات:", "  • 33/33 اختبار ناجح", "  • 20/20 تنفيذ متطابق", "  • zero warnings"]},
            {"command": "echo", "args": ["الخطوة التالية: نشر على GitHub"]}
        ]
    }"#;

    let validator = Validator::new(ValidationConfig::default());
    let validated = validator.validate(mission_json)?;
    let mission_hash = validated.mission_hash();

    let mut executor = MissionExecutor::new(WorkspaceMode::ReadOnly, &mission_hash)?;

    // نسخ مسار workspace قبل التنفيذ (لأنه سيحذف بعدها)
    let _workspace_path = executor.workspace.path().to_path_buf();

    let report = executor.execute(validated)?;

    println!("\n✅ Execution Complete");
    println!("   • Workspace: {}", executor.workspace.uuid());
    println!("   • Hash: {}", &report.final_hash[..16]);
    println!("   • Ticks: {}", report.logical_ticks);

    // ✅ قراءة وعرض الـ facts (بعد التنفيذ وقبل الحذف)
    println!("\n📋 سجل التدقيق (facts.jsonl):");
    println!("----------------------------------------");

    if report.facts_file.exists() {
        let facts = fs::read_to_string(&report.facts_file)
            .unwrap_or_else(|e| format!("خطأ في قراءة الملف: {}", e));

        for line in facts.lines() {
            println!("{}", line);
        }
    } else {
        println!("ملف facts غير موجود (ربما حُذف قبل القراءة)");
    }

    Ok(())
}
