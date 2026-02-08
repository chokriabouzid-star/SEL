//! Cryptographic Hash Chain for Facts
//!
//! Implements the algorithm from facts-schema-v1.md

use sha2::{Sha256, Digest};
use serde_json::Value;
use crate::canonical::canonicalize_json;

pub struct HashChain {
    previous: String,
    counter: usize,
}

impl HashChain {
    /// Create new hash chain
    pub fn new() -> Self {
        Self {
            previous: "0".repeat(64), // Genesis hash
            counter: 0,
        }
    }

    /// Append event and return its hash
    pub fn append(&mut self, event: &Value) -> String {
        // Canonicalize the event
        let canonical = canonicalize_json(event);
        
        // Include position in hash: SHA256(previous_hash + canonical + counter)
        let combined = format!("{}:{}:{}", self.previous, canonical, self.counter);
        
        // Compute hash
        let mut hasher = Sha256::new();
        hasher.update(combined.as_bytes());
        let event_hash = format!("{:x}", hasher.finalize());
        
        // Update chain state
        self.previous = event_hash.clone();
        self.counter += 1;
        
        event_hash
    }

    /// Get final chain hash (for mission_end)
    pub fn finalize(&self) -> String {
        self.previous.clone()
    }
    
    /// Get number of events in chain
    pub fn len(&self) -> usize {
        self.counter
    }
    
    /// Check if chain is empty
    pub fn is_empty(&self) -> bool {
        self.counter == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_genesis_hash() {
        let chain = HashChain::new();
        assert_eq!(chain.finalize(), "0".repeat(64));
        assert_eq!(chain.len(), 0);
        assert!(chain.is_empty());
    }

    #[test]
    fn test_hash_length() {
        let mut chain = HashChain::new();
        let hash = chain.append(&json!({"test": "value"}));
        
        assert_eq!(hash.len(), 64); // SHA-256 hex length
    }
    
    #[test]
    fn test_deterministic_chain() {
        // Same events should produce same chain
        let event = json!({"action": "test"});
        
        let mut chain1 = HashChain::new();
        let hash1 = chain1.append(&event);
        
        let mut chain2 = HashChain::new();
        let hash2 = chain2.append(&event);
        
        assert_eq!(hash1, hash2, "Same event should produce same hash");
        assert_eq!(chain1.finalize(), chain2.finalize(), "Chains should match");
    }
    
    #[test]
    fn test_chain_position_changes() {
        let mut chain = HashChain::new();
        
        let event = json!({"index": 1});
        
        let hash1 = chain.append(&event);
        let position1 = chain.len();
        
        let hash2 = chain.append(&event);
        let position2 = chain.len();
        
        // Hashes should be different because positions are different
        assert_ne!(hash1, hash2, "Same event at different positions should have different hashes");
        assert_eq!(position1, 1);
        assert_eq!(position2, 2);
    }
    
    #[test]
    fn test_tamper_detection() {
        let mut chain1 = HashChain::new();
        let mut chain2 = HashChain::new();
        
        // Chain 1: A, B
        let event_a = json!({"data": "A"});
        let event_b = json!({"data": "B"});
        
        chain1.append(&event_a);
        let hash_correct = chain1.append(&event_b);
        
        // Chain 2: B only (tampered - skipped A)
        let hash_tampered = chain2.append(&event_b);
        
        // Different sequences should produce different hashes
        assert_ne!(hash_correct, hash_tampered, "Tampering should be detected");
    }
    
    #[test]
    fn test_finalize() {
        let mut chain = HashChain::new();
        
        assert_eq!(chain.finalize(), "0".repeat(64));
        
        chain.append(&json!({"test": 1}));
        chain.append(&json!({"test": 2}));
        
        let final_hash = chain.finalize();
        assert!(!final_hash.is_empty());
        assert_eq!(final_hash.len(), 64);
        
        // finalize should not change state
        assert_eq!(chain.finalize(), final_hash);
    }
}
