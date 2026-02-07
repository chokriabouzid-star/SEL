//! محول بين SEL Engine و SEL Validator

use serde_json;

/// تحقق من مهمة باستخدام sel-validator
pub fn validate_mission(mission: &crate::mission::Mission) -> Result<(), crate::Error> {
    println!("🔍 استخدام SEL Validator (18 قاعدة دستورية)...");
    
    // 1. تحويل Mission من sel-engine إلى JSON
    let json_str = serde_json::to_string(mission)
        .map_err(|e| crate::Error::Validation(format!("فشل في تحويل المهمة إلى JSON: {}", e)))?;
    
    println!("📊 تم تحويل المهمة إلى JSON ({} bytes)", json_str.len());
    
    // 2. استخدام Validator من sel-validator
    let validator = sel_validator::Validator::new();
    
    // 3. التحقق باستخدام validate_json (الطريقة الأسهل)
    let result = validator.validate_json(&json_str)
        .map_err(|e| crate::Error::Validation(format!("فشل في تحليل JSON: {}", e)))?;
    
    println!("📋 الحكم: {:?}", result.verdict);
    println!("📊 القواعد: {}/{} ناجحة", result.rules_passed, result.rules_applied);
    
    // 4. فحص النتيجة
    match result.verdict {
        sel_validator::Verdict::Valid => {
            println!("✅ المهمة صالحة (اجتازت {}/{} قاعدة دستورية)", 
                     result.rules_passed, result.rules_applied);
            Ok(())
        }
        sel_validator::Verdict::Invalid => {
            let mut error_msgs = Vec::new();
            error_msgs.push(format!("❌ المهمة غير صالحة (فشلت في {}/{} قاعدة)", 
                                   result.rules_applied - result.rules_passed, 
                                   result.rules_applied));
            
            // إضافة تفاصيل الانتهاكات
            if !result.violations.is_empty() {
                error_msgs.push("📋 الانتهاكات:".to_string());
                for (i, violation) in result.violations.iter().enumerate() {
                    error_msgs.push(format!("  {}. [{}] {}", 
                                           i + 1, 
                                           violation.rule, 
                                           violation.fact));
                    if !violation.location.is_empty() {
                        error_msgs.push(format!("     📍 الموقع: {}", violation.location));
                    }
                }
            }
            
            Err(crate::Error::Validation(error_msgs.join("\n")))
        }
    }
}
