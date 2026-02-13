//! # Signature Authority Trait
//! SEL Core 1.1 - Pluggable signature schemes
//!
//! 🔵 HMAC-SHA256: Core 1.0 compatible, deterministic
//! 🔴 Ed25519: Core 1.1, non-repudiation (optional)

use sel_common::{SovereignError, SelResult};

/// Core signature trait for SEL validation proofs
/// Allows pluggable signature schemes without breaking API
pub trait SignatureAuthority {
    /// Sign a payload, return signature as hex string
    fn sign(&self, payload: &[u8]) -> SelResult<String>;
    
    /// Verify a signature against payload
    fn verify(&self, payload: &[u8], signature: &str) -> SelResult<()>;
    
    /// Get the algorithm name for this authority
    fn algorithm(&self) -> &'static str;
}
