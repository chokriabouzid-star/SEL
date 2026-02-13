#![no_main]

use libfuzzer_sys::fuzz_target;
use sel_common::{canonicalize_json_value, SovereignError};
use serde_json::Value;

/// التأكد من عدم وجود أعداد غير حتمية في الـ canonical output
fn assert_no_nondeterministic_numbers(value: &Value) {
    match value {
        Value::Number(n) => {
            assert!(
                n.is_i64() || n.is_u64(),
                "Found non-deterministic number: {}",
                n
            );
        }
        Value::Array(arr) => {
            for v in arr {
                assert_no_nondeterministic_numbers(v);
            }
        }
        Value::Object(obj) => {
            for v in obj.values() {
                assert_no_nondeterministic_numbers(v);
            }
        }
        _ => {}
    }
}

fuzz_target!(|data: &[u8]| {
    // 1️⃣ محاولة parse كـ JSON
    if let Ok(value) = serde_json::from_slice::<Value>(data) {
        
        // 2️⃣ محاولة canonicalize
        match canonicalize_json_value(&value) {
            Ok(canonical) => {
                // ✅ PROPERTY 1: Canonicalization is idempotent
                let canonical2 = canonicalize_json_value(&canonical).unwrap();
                assert_eq!(canonical, canonical2, "Canonicalization not idempotent");
                
                // ✅ PROPERTY 2: Canonical form sorts keys alphabetically
                if let Value::Object(obj) = &canonical {
                    let keys: Vec<_> = obj.keys().collect();
                    let mut sorted_keys = keys.clone();
                    sorted_keys.sort();
                    assert_eq!(keys, sorted_keys, "Keys not sorted");
                }
                
                // ✅ PROPERTY 3: No floats or big ints in output
                assert_no_nondeterministic_numbers(&canonical);
            }
            Err(SovereignError::NonDeterministicNumber) => {
                // ✅ هذا متوقع - input يحتوي على float أو big int
                // لا نفعل شيء، هذا خطأ متوقع
            }
            Err(e) => {
                // ✅ أي خطأ آخر غير متوقع
                panic!("Unexpected error: {}", e);
            }
        }
    }
});
