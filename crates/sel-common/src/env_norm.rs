//! # Environment Normalization
//! SEL Extended 1.1 - Deterministic Environment Filtering
//! 🔴 ONLY normalize known path fields, not arbitrary strings

use std::collections::BTreeMap;
use std::env;

/// Only keep SEL_* environment variables
/// Sort them deterministically by key
pub fn normalize_environment() -> BTreeMap<String, String> {
    let mut result = BTreeMap::new();
    
    for (key, value) in env::vars() {
        if key.starts_with("SEL_") {
            result.insert(key, value);
        }
    }
    
    result
}

/// Normalize a path for Windows/Linux compatibility
/// 🔴 ONLY call this on actual paths, not arbitrary strings
pub fn normalize_path_field(path: &str) -> String {
    // Convert Windows backslashes to forward slashes
    // This is SAFE because this function is ONLY called on path fields
    path.replace('\\', "/")
}

/// Check if a string is likely a filesystem path
/// Used to guard normalize_path_field
pub fn is_likely_path(s: &str) -> bool {
    s.contains('/') || 
    s.contains('\\') || 
    s.starts_with('.') || 
    s.starts_with('~') ||
    cfg!(windows) && s.contains(':')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_path_field() {
        assert_eq!(normalize_path_field("a\\b\\c"), "a/b/c");
        assert_eq!(normalize_path_field("a/b/c"), "a/b/c");
    }
    
    #[test]
    fn test_is_likely_path() {
        assert!(is_likely_path("a/b/c"));
        assert!(is_likely_path("a\\b\\c"));
        assert!(is_likely_path("./src"));
        assert!(is_likely_path("~/project"));
        assert!(!is_likely_path("echo"));
        assert!(!is_likely_path("Hello world"));
    }
    
    #[test]
    fn test_normalize_environment() {
        // 🔴 UNSAFE: modifying environment variables requires unsafe block
        unsafe {
            env::set_var("SEL_TEST", "value");
            env::set_var("NORMAL_VAR", "should be ignored");
        }
        
        let normalized = normalize_environment();
        assert!(normalized.contains_key("SEL_TEST"));
        assert!(!normalized.contains_key("NORMAL_VAR"));
        
        // Clean up
        unsafe {
            env::remove_var("SEL_TEST");
            env::remove_var("NORMAL_VAR");
        }
    }
}
