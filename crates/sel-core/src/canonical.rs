//! Canonical JSON serialization
//! 
//! 10-step canonicalization algorithm for deterministic JSON serialization

use serde_json::Value;

/// Canonicalize JSON according to SEL specification
pub fn canonicalize_json(value: &Value) -> String {
    let mut result = String::new();
    canonicalize_value(value, &mut result, 0);
    result
}

fn canonicalize_value(value: &Value, output: &mut String, depth: usize) {
    match value {
        Value::Null => output.push_str("null"),
        Value::Bool(b) => output.push_str(&b.to_string()),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                output.push_str(&i.to_string());
            } else if let Some(f) = n.as_f64() {
                // Ensure no scientific notation
                output.push_str(&format!("{}", f));
            } else {
                output.push_str(&n.to_string());
            }
        },
        Value::String(s) => {
            output.push('"');
            for ch in s.chars() {
                match ch {
                    '"' => output.push_str("\\\""),
                    '\\' => output.push_str("\\\\"),
                    '\n' => output.push_str("\\n"),
                    '\r' => output.push_str("\\r"),
                    '\t' => output.push_str("\\t"),
                    '\x08' => output.push_str("\\b"),
                    '\x0c' => output.push_str("\\f"),
                    _ => output.push(ch),
                }
            }
            output.push('"');
        },
        Value::Array(arr) => {
            output.push('[');
            for (i, item) in arr.iter().enumerate() {
                if i > 0 {
                    output.push(',');
                }
                canonicalize_value(item, output, depth + 1);
            }
            output.push(']');
        },
        Value::Object(obj) => {
            output.push('{');
            
            // Step 1: Collect and sort keys
            let mut keys: Vec<&String> = obj.keys().collect();
            keys.sort();
            
            for (i, key) in keys.iter().enumerate() {
                if i > 0 {
                    output.push(',');
                }
                
                // Serialize key
                output.push('"');
                for ch in key.chars() {
                    match ch {
                        '"' => output.push_str("\\\""),
                        '\\' => output.push_str("\\\\"),
                        _ => output.push(ch),
                    }
                }
                output.push('"');
                output.push(':');
                
                // Serialize value
                canonicalize_value(&obj[*key], output, depth + 1);
            }
            output.push('}');
        },
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
        let arr = json!([1, 2, 3]);
        assert_eq!(canonicalize_json(&arr), "[1,2,3]");
    }

    #[test]
    fn test_canonicalize_object() {
        let obj = json!({
            "z": 1,
            "a": 2,
            "m": 3
        });
        assert_eq!(canonicalize_json(&obj), "{\"a\":2,\"m\":3,\"z\":1}");
    }

    #[test]
    fn test_canonicalize_complex() {
        let complex = json!({
            "name": "test",
            "values": [1, 2, 3],
            "nested": {
                "inner": "value"
            }
        });
        let result = canonicalize_json(&complex);
        assert!(result.contains("\"name\":\"test\""));
        assert!(result.contains("\"values\":[1,2,3]"));
    }

    #[test]
    fn test_canonicalize_consistent() {
        let obj1 = json!({
            "b": 2,
            "a": 1,
            "c": 3
        });
        let obj2 = json!({
            "c": 3,
            "a": 1,
            "b": 2
        });
        
        let result1 = canonicalize_json(&obj1);
        let result2 = canonicalize_json(&obj2);
        
        assert_eq!(result1, result2);
    }
}
