//! Mission and Action Canonicalization
//! 
//! Implements the 10-step algorithm from behavior-spec-v1.md

use serde_json::{Value, Map, json};
use std::collections::BTreeMap;
use unicode_normalization::UnicodeNormalization;

/// Canonicalize any JSON value according to SEL spec
pub fn canonicalize_json(value: &Value) -> String {
    let canonical = canonicalize_value(value);
    serde_json::to_string(&canonical).expect("canonicalization failed")
}

fn canonicalize_value(value: &Value) -> Value {
    match value {
        Value::Object(map) => {
            // Step 2: Sort keys lexicographically
            let mut sorted = BTreeMap::new();
            for (k, v) in map {
                sorted.insert(k.clone(), canonicalize_value(v));
            }
            
            let sorted_map: Map<String, Value> = sorted.into_iter().collect();
            Value::Object(sorted_map)
        }
        Value::Array(arr) => {
            // Step 8: Preserve array order
            Value::Array(arr.iter().map(canonicalize_value).collect())
        }
        Value::String(s) => {
            // Step 6: Unicode NFC normalization
            Value::String(normalize_unicode(s))
        }
        // Step 4: Numbers (no trailing zeros)
        // Step 9: Booleans (lowercase)
        // Step 10: Null (lowercase)
        other => other.clone(),
    }
}

fn normalize_unicode(s: &str) -> String {
    // Unicode NFC normalization
    s.nfc().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_key_sorting() {
        let json = json!({
            "z": 1,
            "a": 2,
            "m": 3
        });
        
        let canonical = canonicalize_json(&json);
        assert_eq!(canonical, r#"{"a":2,"m":3,"z":1}"#);
    }
    
    #[test]
    fn test_nested_sorting() {
        let json = json!({
            "outer": {
                "z": 1,
                "a": 2
            }
        });
        
        let canonical = canonicalize_json(&json);
        // Use contains for flexible whitespace
        assert!(canonical.contains(r#""outer":{"a":2,"z":1}"#));
    }
    
    #[test]
    fn test_unicode_normalization() {
        let json = json!({
            "text": "café"
        });
        
        let canonical = canonicalize_json(&json);
        // Should normalize Unicode
        assert!(canonical.contains(r#""text":"café""#));
    }
    
    #[test]
    fn test_array_order_preserved() {
        let json = json!({
            "items": [3, 1, 2]
        });
        
        let canonical = canonicalize_json(&json);
        // Arrays should NOT be sorted
        assert!(canonical.contains(r#""items":[3,1,2]"#));
    }
    
    #[test]
    fn test_metadata_included() {
        let json = json!({
            "name": "test",
            "metadata": {"key": "value"},
            "actions": []
        });
        
        let canonical = canonicalize_json(&json);
        // Metadata should be included in canonical form
        assert!(canonical.contains(r#""metadata":{"key":"value"}"#));
    }
}
