//! Adapter between sel-core and sel-engine
//!
//! Provides convenience functions for canonicalization and hashing

use sel_core::{canonicalize_json, HashChain};
use serde_json::Value;

/// Canonicalize and hash a mission JSON string
pub fn canonicalize_mission(mission_json: &str) -> Result<(String, String), String> {
    // Parse JSON
    let mission: Value = serde_json::from_str(mission_json)
        .map_err(|e| format!("Invalid JSON: {}", e))?;

    // Canonicalize
    let canonical = canonicalize_json(&mission);

    // Hash
    use sha2::{Sha256, Digest};
    let mut hasher = Sha256::new();
    hasher.update(canonical.as_bytes());
    let hash = format!("sha256:{:x}", hasher.finalize());

    Ok((canonical, hash))
}

/// Create a new hash chain
pub fn create_hash_chain() -> HashChain {
    HashChain::new()
}

/// Hash arbitrary data with SHA-256
pub fn hash_data(data: &[u8]) -> String {
    use sha2::{Sha256, Digest};
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_canonicalize_mission() {
        let mission = r#"{"name":"test","actions":[]}"#;
        let result = canonicalize_mission(mission);
        
        assert!(result.is_ok());
        let (canonical, hash) = result.unwrap();
        
        assert!(!canonical.is_empty());
        assert!(hash.starts_with("sha256:"));
        assert_eq!(hash.len(), 71); // "sha256:" + 64 hex chars
    }

    #[test]
    fn test_deterministic_hash() {
        let mission = r#"{"name":"test","actions":[]}"#;
        
        let (_, hash1) = canonicalize_mission(mission).unwrap();
        let (_, hash2) = canonicalize_mission(mission).unwrap();
        
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_create_hash_chain() {
        let chain = create_hash_chain();
        assert_eq!(chain.len(), 0);
    }
}
