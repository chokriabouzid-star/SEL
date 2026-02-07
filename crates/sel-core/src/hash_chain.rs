//! Hash chain for tamper-proof event logging

use sha2::{Sha256, Digest};
use serde_json::Value;

/// Hash chain for sequential event verification
pub struct HashChain {
    previous_hash: String,
    events: Vec<String>,
}

impl HashChain {
    /// Create new hash chain
    pub fn new() -> Self {
        HashChain {
            previous_hash: String::from("sel_chain_init"),
            events: Vec::new(),
        }
    }
    
    /// Append event to chain and return its hash
    pub fn append(&mut self, event: &Value) -> String {
        use crate::canonicalize_json;
        
        // Canonicalize event
        let canonical = canonicalize_json(event);
        
        // Combine with previous hash
        let combined = format!("{}:{}", self.previous_hash, canonical);
        
        // Compute hash
        let event_hash = sha256_hex(combined.as_bytes());
        
        // Update chain
        self.previous_hash = event_hash.clone();
        self.events.push(canonical);
        
        event_hash
    }
    
    /// Get final chain hash
    pub fn finalize(&self) -> String {
        self.previous_hash.clone()
    }
    
    /// Get number of events in chain
    pub fn len(&self) -> usize {
        self.events.len()
    }
    
    /// Check if chain is empty
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }
}

/// Compute SHA-256 hash in hex format
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
    fn test_hash_chain_sequential() {
        let mut chain = HashChain::new();
        
        let event1 = json!({"action": "start", "timestamp": 123});
        let hash1 = chain.append(&event1);
        
        let event2 = json!({"action": "end", "timestamp": 456});
        let hash2 = chain.append(&event2);
        
        assert_ne!(hash1, hash2);
        assert_eq!(chain.len(), 2);
    }
    
    #[test]
    fn test_hash_chain_deterministic() {
        let mut chain1 = HashChain::new();
        let mut chain2 = HashChain::new();
        
        let event = json!({"test": "same"});
        
        let hash1 = chain1.append(&event);
        let hash2 = chain2.append(&event);
        
        assert_eq!(hash1, hash2, "Same event should produce same hash");
    }
    
    #[test]
    fn test_finalize() {
        let mut chain = HashChain::new();
        
        chain.append(&json!({"a": 1}));
        chain.append(&json!({"b": 2}));
        
        let final_hash = chain.finalize();
        assert!(!final_hash.is_empty());
        assert_eq!(final_hash.len(), 64); // SHA-256 hex length
    }
}
