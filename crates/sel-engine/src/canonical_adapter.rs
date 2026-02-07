//! Adapter between sel-core and sel-engine
//! 
//! Provides mission canonicalization and hashing

use sel_core::canonicalize_json;
use sha2::{Sha256, Digest};
use serde_json;

/// Canonicalize mission and compute its hash
pub fn canonicalize_mission(mission: &serde_json::Value) -> (String, String) {
    // Step 1: Canonicalize using sel-core
    let canonical = canonicalize_json(mission);
    
    // Step 2: Compute SHA-256 hash
    let hash = sha256_hex(canonical.as_bytes());
    
    (canonical, hash)
}

/// Compute SHA-256 hash in hex format
fn sha256_hex(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}

/// Create hash chain for mission facts
pub fn create_hash_chain() -> sel_core::HashChain {
    sel_core::HashChain::new()
}

/// Format mission hash with prefix
pub fn format_mission_hash(hash: &str) -> String {
    format!("sha256:{}", hash)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    
    #[test]
    fn test_canonicalize_simple_mission() {
        let mission = json!({
            "name": "test",
            "actions": [
                {"type": "command", "command": "echo hello"}
            ]
        });
        
        let (canonical, hash) = canonicalize_mission(&mission);
        assert!(!canonical.is_empty());
        assert_eq!(hash.len(), 64); // SHA-256 hex length
    }
    
    #[test]
    fn test_deterministic_hashing() {
        let mission = json!({
            "name": "deterministic-test",
            "actions": []
        });
        
        let (_, hash1) = canonicalize_mission(&mission);
        let (_, hash2) = canonicalize_mission(&mission);
        
        assert_eq!(hash1, hash2, "Same mission should produce same hash");
    }
}
