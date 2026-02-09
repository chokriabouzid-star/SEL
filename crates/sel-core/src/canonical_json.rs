//! Canonical JSON implementation

use serde_json::{Value, Map};
use std::collections::BTreeMap;

/// Canonicalize JSON according to RFC 8785
pub fn canonicalize_json(value: &Value) -> String {
    match value {
        Value::Null => "null".to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Number(n) => n.to_string(),
        Value::String(s) => {
            let escaped = s.escape_default().to_string();
            format!("\"{}\"", escaped)
        }
        Value::Array(arr) => {
            let items: Vec<String> = arr.iter().map(canonicalize_json).collect();
            format!("[{}]", items.join(","))
        }
        Value::Object(obj) => {
            let mut sorted = BTreeMap::new();
            for (k, v) in obj {
                sorted.insert(k, canonicalize_json(v));
            }
            let items: Vec<String> = sorted
                .iter()
                .map(|(k, v)| format!("\"{}\":{}", k.escape_default(), v))
                .collect();
            format!("{{{}}}", items.join(","))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    
    #[test]
    fn test_canonicalize_basic_types() {
        assert_eq!(canonicalize_json(&json!(null)), "null");
        assert_eq!(canonicalize_json(&json!(true)), "true");
        assert_eq!(canonicalize_json(&json!(false)), "false");
        assert_eq!(canonicalize_json(&json!(42)), "42");
        assert_eq!(canonicalize_json(&json!(3.14)), "3.14");
        assert_eq!(canonicalize_json(&json!("hello")), "\"hello\"");
    }
    
    #[test]
    fn test_canonicalize_array() {
        let input = json!([1, 2, 3]);
        assert_eq!(canonicalize_json(&input), "[1,2,3]");
    }
    
    #[test]
    fn test_canonicalize_object() {
        let input = json!({"b": 2, "a": 1});
        // Keys should be sorted alphabetically
        assert_eq!(canonicalize_json(&input), "{\"a\":1,\"b\":2}");
    }
    
    #[test]
    fn test_canonicalize_complex() {
        let input = json!({
            "z": [1, 2],
            "a": {"c": 3, "b": 2},
            "m": "test"
        });
        let result = canonicalize_json(&input);
        // Should be sorted by keys
        assert!(result.starts_with("{\"a\":"));
        assert!(result.contains("\"m\":\"test\""));
        assert!(result.contains("\"z\":[1,2]"));
    }
    
    #[test]
    fn test_canonicalize_consistent() {
        let input1 = json!({"b": 2, "a": 1});
        let input2 = json!({"a": 1, "b": 2});
        
        let output1 = canonicalize_json(&input1);
        let output2 = canonicalize_json(&input2);
        
        assert_eq!(output1, output2, "Different key order should produce same canonical JSON");
    }
    
    #[test]
    fn test_canonicalize_string_escape() {
        let input = json!("line1\nline2");
        let result = canonicalize_json(&input);
        assert_eq!(result, "\"line1\\nline2\"");
    }
}
