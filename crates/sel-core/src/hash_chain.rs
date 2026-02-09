//! Hash Chain implementation

use sha2::{Sha256, Digest};
use serde_json::Value;
use crate::canonicalize_json;

/// Hash chain for deterministic verification
#[derive(Debug, Clone)]
pub struct HashChain {
    chain: Vec<String>,
}

impl HashChain {
    /// Create new hash chain
    pub fn new() -> Self {
        Self { chain: Vec::new() }
    }
    
    /// Append data to chain and return hash
    pub fn append(&mut self, data: &Value) -> String {
        let canonical = canonicalize_json(data);
        let mut hasher = Sha256::new();
        
        // Include previous hash if exists
        if let Some(last) = self.chain.last() {
            hasher.update(last.as_bytes());
        }
        
        hasher.update(canonical.as_bytes());
        let result = hasher.finalize();
        let hash = hex::encode(result);
        
        self.chain.push(hash.clone());
        hash
    }
    
    /// Get current chain
    pub fn chain(&self) -> &[String] {
        &self.chain
    }
    
    /// Get final hash
    pub fn final_hash(&self) -> Option<&str> {
        self.chain.last().map(|s| s.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    
    #[test]
    fn test_deterministic_chain() {
        let event = json!({"action": "test"});
        
        let mut chain1 = HashChain::new();
        let mut chain2 = HashChain::new();
        
        // Append same event to both chains
        let hash1 = chain1.append(&event);
        let hash2 = chain2.append(&event);
        
        assert_eq!(hash1, hash2, "Same event should produce same chain hash");
    }
    
    #[test]
    fn test_chain_position_changes() {
        let event1 = json!({"step": 1});
        let event2 = json!({"step": 2});
        
        let mut chain_a = HashChain::new();
        chain_a.append(&event1);
        let hash_a = chain_a.append(&event2);
        
        let mut chain_b = HashChain::new();
        chain_b.append(&event2);
        let hash_b = chain_b.append(&event1);
        
        assert_ne!(hash_a, hash_b, "Order should matter in hash chain");
    }
    
    #[test]
    fn test_genesis_hash() {
        let mut chain = HashChain::new();
        let event = json!({"genesis": true});
        
        let hash = chain.append(&event);
        assert!(!hash.is_empty(), "Genesis hash should not be empty");
        assert_eq!(chain.chain().len(), 1, "Chain should have one item");
    }
    
    #[test]
    fn test_hash_length() {
        let mut chain = HashChain::new();
        let event = json!({"test": "data"});
        
        let hash = chain.append(&event);
        assert_eq!(hash.len(), 64, "SHA256 hash should be 64 hex chars");
    }
    
    #[test]
    fn test_tamper_detection() {
        let mut chain = HashChain::new();
        
        let original = json!({"data": "original"});
        let modified = json!({"data": "modified"});
        
        let hash1 = chain.append(&original);
        let hash2 = chain.append(&modified);
        
        assert_ne!(hash1, hash2, "Different data should produce different hashes");
    }
    
    #[test]
    fn test_final_hash() {
        let mut chain = HashChain::new();
        let event1 = json!({"step": 1});
        let event2 = json!({"step": 2});
        
        chain.append(&event1);
        chain.append(&event2);
        
        assert_eq!(chain.chain().len(), 2);
        assert!(chain.final_hash().is_some());
        
        let final_hash = chain.final_hash().unwrap();
        assert_eq!(final_hash.len(), 64, "Final hash should be 64 hex chars");
    }
}
