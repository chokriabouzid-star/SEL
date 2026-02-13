//! # Environment Normalization
//! SEL Core 1.0 - Deterministic Environment Filtering
//!
//! 🔴 THIS MODULE IS NOT USED IN CANONICAL PIPELINE
//! It is ONLY for collecting SEL_* environment variables
//! No paths are normalized during hashing/validation

use std::collections::BTreeMap;
use std::env;

/// Only keep SEL_* environment variables
/// Sort them deterministically by key
/// 
/// 🔴 DETERMINISTIC: Same keys/values on all platforms
/// Not used in hash chain - only for workspace configuration
pub fn normalize_environment() -> BTreeMap<String, String> {
    let mut result = BTreeMap::new();
    
    for (key, value) in env::vars() {
        if key.starts_with("SEL_") {
            result.insert(key, value);
        }
    }
    
    result
}

/// Check if a string is likely a filesystem path
/// 🔴 NOT USED IN CANONICAL PIPELINE
/// Only for workspace path handling (not part of hash)
pub fn is_likely_path(s: &str) -> bool {
    s.contains('/') || 
    s.contains('\\') || 
    s.starts_with('.') || 
    s.starts_with('~') ||
    s.contains(':')      // ✅ Same behavior on all platforms
}

/// Normalize a path for cross-platform compatibility
/// 🔴 ONLY for workspace paths - NOT part of deterministic hash
pub fn normalize_path_field(path: &str) -> String {
    path.replace('\\', "/")
}

/// Safe path normalization with guard
/// 🔴 ONLY for workspace paths - NOT part of deterministic hash
pub fn normalize_path_safe(path: &str) -> String {
    if is_likely_path(path) {
        normalize_path_field(path)
    } else {
        path.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_likely_path_deterministic() {
        // Same behavior on all platforms
        assert!(is_likely_path("a/b/c"));
        assert!(is_likely_path("a\\b\\c"));
        assert!(is_likely_path("./src"));
        assert!(is_likely_path("~/project"));
        assert!(is_likely_path("C:/Windows"));
        assert!(is_likely_path("C:\\Windows"));
        
        assert!(!is_likely_path("echo"));
        assert!(!is_likely_path("Hello world"));
    }
}
