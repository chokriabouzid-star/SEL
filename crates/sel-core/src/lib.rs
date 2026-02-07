//! SEL Core - Sovereign DNA
//! 
//! Provides canonicalization, hashing, and environment normalization

pub mod canonical;
pub mod hash_chain;
pub mod env_norm;

pub use canonical::canonicalize_json;
pub use hash_chain::HashChain;
pub use env_norm::{normalize_command_env, validate_user_env};

#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[test]
    fn test_full_pipeline() {
        // 1. Canonicalize mission
        let mission = serde_json::json!({
            "name": "test",
            "actions": []
        });
        
        let canonical = canonicalize_json(&mission);
        
        // 2. Hash chain
        let mut chain = HashChain::new();
        let event = serde_json::json!({
            "type": "mission_start",
            "mission_hash": canonical
        });
        
        let hash = chain.append(&event);
        
        assert_eq!(hash.len(), 64); // SHA-256 hex
    }
}
