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

    /// Append event and return its CHAIN hash (previous + event)
    pub fn append(&mut self, event: &Value) -> String {
        self.counter += 1;
        
        // Canonicalize the event
        let canonical = canonicalize_json(event);

        // Hash = SHA256(canonical_bytes) - this is the EVENT hash
        let event_hash = sha256_hex(canonical.as_bytes());

        // Chain = SHA256(previous || event_hash || counter) 
        // Added counter to ensure different positions give different hashes
        let chain_input = format!("{}{}{}", self.previous, event_hash, self.counter);
        let chain_hash = sha256_hex(chain_input.as_bytes());

        // Update state
        self.previous = chain_hash.clone();

        // Return CHAIN hash (not event hash) 
        // This includes position (counter) and previous chain state
        chain_hash
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
    
    /// Get event hash (for logging individual events)
    pub fn event_hash(event: &Value) -> String {
        let canonical = canonicalize_json(event);
        sha256_hex(canonical.as_bytes())
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
        assert_eq!(chain.previous, "0".repeat(64));
        assert_eq!(chain.len(), 0);
    }

    #[test]
    fn test_deterministic_chain() {
        let event = json!({"type": "test", "data": 123});

        let mut chain1 = HashChain::new();
        let hash1 = chain1.append(&event);

        let mut chain2 = HashChain::new();
        let hash2 = chain2.append(&event);

        assert_eq!(hash1, hash2, "Same event should produce same chain hash");
        assert_eq!(chain1.len(), 1);
        assert_eq!(chain2.len(), 1);
    }

    #[test]
    fn test_tamper_detection() {
        let event1 = json!({"type": "a"});
        let event2 = json!({"type": "b"});

        // Correct chain: event1 then event2
        let mut chain = HashChain::new();
        let _ = chain.append(&event1);
        let hash_correct = chain.append(&event2);

        // Tampered chain: only event2 (skipping event1)
        let mut chain_tampered = HashChain::new();
        let hash_tampered = chain_tampered.append(&event2);

        // Hashes should be different because:
        // - hash_correct uses: SHA256(genesis→event1→event2)
        // - hash_tampered uses: SHA256(genesis→event2)
        assert_ne!(
            hash_correct, 
            hash_tampered,
            "Tampering should be detected: missing event changes chain hash"
        );
    }

    #[test]
    fn test_chain_position_changes() {
        let event = json!({"type": "test"});

        let mut chain = HashChain::new();

        let pos1 = chain.append(&event);  // position 1: SHA256(genesis || event_hash || 1)
        let pos2 = chain.append(&event);  // position 2: SHA256(pos1 || event_hash || 2)
        let pos3 = chain.append(&event);  // position 3: SHA256(pos2 || event_hash || 3)

        // Same event, different positions → DIFFERENT chain hashes
        assert_ne!(pos1, pos2, "Position 1 != Position 2");
        assert_ne!(pos2, pos3, "Position 2 != Position 3");
        assert_ne!(pos1, pos3, "Position 1 != Position 3");

        assert_eq!(chain.len(), 3);
    }

    #[test]
    fn test_hash_length() {
        let mut chain = HashChain::new();
        let hash = chain.append(&json!({}));
        assert_eq!(hash.len(), 64); // SHA-256 hex should be 64 chars
    }

    #[test]
    fn test_finalize() {
        let mut chain = HashChain::new();
        let hash1 = chain.append(&json!({"a": 1}));
        let hash2 = chain.append(&json!({"b": 2}));

        let final_hash = chain.finalize();
        assert_eq!(final_hash, hash2, "finalize() should return last chain hash");
        assert_eq!(final_hash.len(), 64);

        // Finalize should be idempotent
        assert_eq!(final_hash, chain.finalize());
    }
}
