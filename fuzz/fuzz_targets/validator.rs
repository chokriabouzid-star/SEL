#![no_main]

use libfuzzer_sys::fuzz_target;
use sel_validator::{Validator, ValidationConfig};
use sel_common::SovereignError;

fuzz_target!(|data: &[u8]| {
    // محاولة تفسير الـ fuzz data كـ JSON
    if let Ok(mission_str) = std::str::from_utf8(data) {
        // تجاهل الـ JSON الطويلة جداً (حماية من DoS)
        if mission_str.len() > 10_000 {
            return;
        }
        
        // إنشاء validator مع config افتراضي
        let config = ValidationConfig::default();
        if let Ok(validator) = Validator::new(config) {
            
            // ✅ PROPERTY 1: Validate should never panic
            let result1 = validator.validate(mission_str);
            
            // ✅ PROPERTY 2: Same input = Same output (determinism)
            let result2 = validator.validate(mission_str);
            
            match (result1, result2) {
                (Ok(validated1), Ok(validated2)) => {
                    assert_eq!(
                        validated1.mission_hash(),
                        validated2.mission_hash(),
                        "Non-deterministic validation: different hashes for same input"
                    );
                }
                (Err(e1), Err(e2)) => {
                    // نفس الخطأ يجب أن يظهر
                    assert_eq!(
                        format!("{}", e1),
                        format!("{}", e2),
                        "Non-deterministic validation: different errors for same input"
                    );
                }
                _ => panic!("Determinism violation: Ok vs Err for same input"),
            }
        }
    }
});
