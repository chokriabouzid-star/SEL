//! # Hash Chain - Tamper-Evident Fact Chain
//! SEL Core 1.0 - SHA-256 Based
//! DETERMINISTIC: Same facts = Same final hash

use serde_json::Value;
use sha2::{Digest, Sha256};

/// GENESIS HASH - ثابت وغير قابل للتغيير
/// SEL Core 1.0: يجب أن يكون هذا ثابتاً في كل التنفيذات
pub const GENESIS_HASH: &str = "0000000000000000000000000000000000000000000000000000000000000000";

/// Hash chain with deterministic final hash
/// Same sequence of facts = Same final hash, ALWAYS
pub struct HashChain {
    previous_hash: String,
    chain: Vec<String>,
}

impl Default for HashChain {
    fn default() -> Self {
        Self::new()
    }
}

impl HashChain {
    pub fn new() -> Self {
        Self {
            previous_hash: GENESIS_HASH.to_string(),
            chain: Vec::new(),
        }
    }

    /// Add fact to chain and return its hash
    /// DETERMINISTIC: Same fact + same previous_hash = same output
    pub fn add_fact(&mut self, fact: &Value) -> String {
        let fact_json = serde_json::to_string(fact).unwrap();
        let mut hasher = Sha256::new();
        hasher.update(self.previous_hash.as_bytes());
        hasher.update(fact_json.as_bytes());
        let hash = hex::encode(hasher.finalize());

        self.chain.push(hash.clone());
        self.previous_hash = hash.clone();

        hash
    }

    /// Get the current hash (final hash at end of execution)
    pub fn finalize(&self) -> String {
        self.previous_hash.clone()
    }

    /// Verify a chain of facts against the final hash
    pub fn verify_chain(&self, facts: &[Value]) -> bool {
        let mut prev = GENESIS_HASH.to_string();

        for fact in facts {
            let fact_json = serde_json::to_string(fact).unwrap();
            let mut hasher = Sha256::new();
            hasher.update(prev.as_bytes());
            hasher.update(fact_json.as_bytes());
            let hash = hex::encode(hasher.finalize());
            prev = hash;
        }

        prev == self.previous_hash
    }
}

/// Builder for deterministic hash chain
pub struct HashChainBuilder {
    previous_hash: String,
}

impl HashChainBuilder {
    pub fn new() -> Self {
        Self {
            previous_hash: GENESIS_HASH.to_string(),
        }
    }

    pub fn add_fact(&mut self, fact: &Value) -> String {
        let mut hasher = Sha256::new();
        hasher.update(self.previous_hash.as_bytes());
        hasher.update(serde_json::to_string(fact).unwrap().as_bytes());
        let hash = hex::encode(hasher.finalize());
        self.previous_hash = hash.clone();
        hash
    }

    pub fn build(self) -> String {
        self.previous_hash
    }
}

impl Default for HashChainBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_chain_deterministic() {
        let mut chain1 = HashChain::new();
        let mut chain2 = HashChain::new();

        let fact = serde_json::json!({"type": "test", "value": 42});

        chain1.add_fact(&fact);
        chain2.add_fact(&fact);

        assert_eq!(chain1.finalize(), chain2.finalize());
    }

    #[test]
    fn test_genesis_hash_public() {
        assert_eq!(
            GENESIS_HASH,
            "0000000000000000000000000000000000000000000000000000000000000000"
        );
    }
}
