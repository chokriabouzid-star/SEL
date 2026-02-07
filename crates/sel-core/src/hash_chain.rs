//! Cryptographic Hash Chain for Facts
//! 
//! Implements the algorithm from facts-schema-v1.md

use sha2::{Sha256, Digest};
use serde_json::Value;
use crate::canonical::canonicalize_json;

pub struct HashChain {
    previous: String,
}

impl HashChain {
    /// Create new hash chain
    pub fn new() -> Self {
        Self {
            previous: "0".repeat(64), // Genesis hash
        }
    }
    
    /// Append event and return its hash
    pub fn append(&mut self, event: &Value) -> String {
        // Canonicalize the event
        let canonical = canonicalize_json(event);
        
        // Hash = SHA256(canonical_bytes)
        let event_hash = sha256_hex(canonical.as_bytes());
        
        // Chain = SHA256(previous || current)
        let chain_input = format!("{}{}", self.previous, event_hash);
        let chain_hash = sha256_hex(chain_input.as_bytes());
        
        // Update state
        self.previous = chain_hash.clone();
        
        event_hash
    }
    
    /// Get final chain hash (for mission_end)
    pub fn finalize(&self) -> String {
        self.previous.clone()
    }
    
    /// Get current chain position (for testing)
    pub fn current_position(&self) -> &str {
        &self.previous
    }
}

fn sha256_hex(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    
    #[test]
    fn test_genesis_hash() {
        let chain = HashChain::new();
        assert_eq!(chain.previous.len(), 64);
    }
    
    #[test]
    fn test_deterministic_chain() {
        let event = json!({"type": "test", "data": 123});
        
        let mut chain1 = HashChain::new();
        let hash1 = chain1.append(&event);
        
        let mut chain2 = HashChain::new();
        let hash2 = chain2.append(&event);
        
        assert_eq!(hash1, hash2);
    }
    
    #[test]
    fn test_tamper_detection() {
        // This test checks that hash chain position differs
        // when events are processed in different order
        
        let event_a = json!({"type": "a", "value": 1});
        let event_b = json!({"type": "b", "value": 2});
        
        // Chain 1: A then B
        let mut chain1 = HashChain::new();
        chain1.append(&event_a);
        chain1.append(&event_b);
        let final_hash1 = chain1.finalize();
        
        // Chain 2: B then A (tampered order)
        let mut chain2 = HashChain::new();
        chain2.append(&event_b);
        chain2.append(&event_a);
        let final_hash2 = chain2.finalize();
        
        // Different order should produce different chain hash
        assert_ne!(final_hash1, final_hash2, 
            "Hash chain should detect order tampering");
    }
    
    #[test]
    fn test_hash_length() {
        let event = json!({"test": "data"});
        let mut chain = HashChain::new();
        let hash = chain.append(&event);
        
        // SHA-256 hex should be 64 characters
        assert_eq!(hash.len(), 64);
    }
    
    #[test]
    fn test_chain_position_changes() {
        let event1 = json!({"type": "first"});
        let event2 = json!({"type": "second"});
        
        let mut chain = HashChain::new();
        let pos1 = chain.current_position().to_string();
        
        chain.append(&event1);
        let pos2 = chain.current_position().to_string();
        
        chain.append(&event2);
        let pos3 = chain.current_position().to_string();
        
        // Each event should change chain position
        assert_ne!(pos1, pos2);
        assert_ne!(pos2, pos3);
        assert_ne!(pos1, pos3);
    }
}
