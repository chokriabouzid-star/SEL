//! # Canonical JSON - Deterministic Serialization
//! SEL Core 1.0 - RFC 8785 Compatible
//! 🔴 TOTAL FUNCTION: Either valid canonical JSON or SovereignError

use serde_json::{Value, Map};
use std::collections::BTreeMap;
use std::iter::FromIterator;
use crate::{SovereignError, SelResult};

/// Convert any JSON to canonical form (deterministic)
pub fn canonicalize_json(json_str: &str) -> SelResult<String> {
    let value: Value = serde_json::from_str(json_str)
        .map_err(|e| SovereignError::InvalidMissionFormat(
            format!("Invalid JSON: {}", e)
        ))?;
    
    let canonical = canonicalize_json_value(&value)?;
    
    serde_json::to_string(&canonical)
        .map_err(|e| SovereignError::InvalidMissionFormat(
            format!("Failed to serialize canonical JSON: {}", e)
        ))
}

/// Recursively canonicalize a JSON value
/// 🔴 ONLY i64 and u64 are allowed - no floats, no big integers
pub fn canonicalize_json_value(value: &Value) -> SelResult<Value> {
    match value {
        Value::Object(obj) => {
            let mut sorted = BTreeMap::new();
            for (k, v) in obj.iter() {
                sorted.insert(k.clone(), canonicalize_json_value(v)?);
            }
            Ok(Value::Object(Map::from_iter(sorted)))
        }
        Value::Array(arr) => {
            let mut new_arr = Vec::with_capacity(arr.len());
            for v in arr {
                new_arr.push(canonicalize_json_value(v)?);
            }
            Ok(Value::Array(new_arr))
        }
        Value::Number(n) => {
            // 🔴 CRITICAL: Only i64 and u64 are deterministic
            // i128, u128, and floats are rejected
            if n.is_i64() || n.is_u64() {
                Ok(Value::Number(n.clone()))
            } else {
                Err(SovereignError::NonDeterministicNumber)
            }
        }
        _ => Ok(value.clone()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_i64_accepted() {
        let value = json!({"value": 123});
        let result = canonicalize_json_value(&value);
        assert!(result.is_ok());
    }

    #[test]
    fn test_u64_accepted() {
        let value = json!({"value": 18446744073709551615u64});
        let result = canonicalize_json_value(&value);
        assert!(result.is_ok());
    }

    #[test]
    fn test_float_rejected() {
        let value = json!({"value": 1.23});
        let result = canonicalize_json_value(&value);
        assert!(matches!(result, Err(SovereignError::NonDeterministicNumber)));
    }
}
