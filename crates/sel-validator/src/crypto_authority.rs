//! # Cryptographic Authority
//! SEL Core 1.1 - Pluggable Signature Schemes
//!
//! 🔵 HMAC-SHA256: Core 1.0 compatible, deterministic (default)
//! 🔴 Ed25519: Core 1.1, non-repudiation (optional, feature-gated)

use hmac::{Hmac, Mac};
use sha2::Sha256;
use sel_common::{SovereignError, SelResult};
use crate::signature::SignatureAuthority;

type HmacSha256 = Hmac<Sha256>;

#[cfg(feature = "ed25519")]
use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Signer, Verifier};
#[cfg(feature = "ed25519")]
use rand::rngs::OsRng;

/// HMAC-SHA256 implementation - Core 1.0 compatible
/// ✅ Deterministic, no randomness
#[derive(Clone)]
pub struct HmacAuthority {
    key: Vec<u8>,
}

impl HmacAuthority {
    pub fn new(key: Vec<u8>) -> SelResult<Self> {
        if key.is_empty() {
            return Err(SovereignError::InternalError("HMAC key cannot be empty".to_string()));
        }
        Ok(Self { key })
    }
    
    /// Create deterministic test key (Core 1.0)
    pub fn test_key() -> Self {
        Self {
            key: vec![
                0x53, 0x45, 0x4c, 0x5f, 0x43, 0x4f, 0x52, 0x45,
                0x5f, 0x31, 0x2e, 0x30, 0x5f, 0x4b, 0x45, 0x59,
            ],
        }
    }
}

impl SignatureAuthority for HmacAuthority {
    fn sign(&self, payload: &[u8]) -> SelResult<String> {
        let mut mac = HmacSha256::new_from_slice(&self.key)
            .map_err(|_| SovereignError::InternalError("Invalid HMAC key length".to_string()))?;
        
        mac.update(payload);
        let result = mac.finalize();
        Ok(hex::encode(result.into_bytes()))
    }
    
    fn verify(&self, payload: &[u8], signature: &str) -> SelResult<()> {
        let signature_bytes = hex::decode(signature)
            .map_err(|_| SovereignError::InvalidMissionFormat("Invalid signature hex".to_string()))?;
        
        let mut mac = HmacSha256::new_from_slice(&self.key)
            .map_err(|_| SovereignError::InternalError("Invalid HMAC key length".to_string()))?;
        
        mac.update(payload);
        
        mac.verify_slice(&signature_bytes)
            .map_err(|_| SovereignError::InvalidMissionFormat("Invalid signature".to_string()))
    }
    
    fn algorithm(&self) -> &'static str {
        "HMAC-SHA256"
    }
}

/// Ed25519 implementation - Core 1.1, optional feature
/// 🔴 Non-repudiation, public key crypto
#[cfg(feature = "ed25519")]
#[derive(Clone)]
pub struct Ed25519Authority {
    signing_key: SigningKey,
}

#[cfg(feature = "ed25519")]
impl Ed25519Authority {
    pub fn new() -> Self {
        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        Self { signing_key }
    }
    
    pub fn from_bytes(bytes: &[u8; 32]) -> Self {
        let signing_key = SigningKey::from_bytes(bytes);
        Self { signing_key }
    }
    
    pub fn public_key(&self) -> String {
        hex::encode(VerifyingKey::from(&self.signing_key).to_bytes())
    }
}

#[cfg(feature = "ed25519")]
impl SignatureAuthority for Ed25519Authority {
    fn sign(&self, payload: &[u8]) -> SelResult<String> {
        let signature = self.signing_key.sign(payload);
        Ok(hex::encode(signature.to_bytes()))
    }
    
    fn verify(&self, payload: &[u8], signature: &str) -> SelResult<()> {
        let signature_bytes = hex::decode(signature)
            .map_err(|_| SovereignError::InvalidMissionFormat("Invalid signature hex".to_string()))?;
        
        let signature = Signature::from_bytes(&signature_bytes.try_into().map_err(|_| 
            SovereignError::InvalidMissionFormat("Invalid signature length".to_string()))?)
            .map_err(|_| SovereignError::InvalidMissionFormat("Invalid signature format".to_string()))?;
        
        let verifying_key = VerifyingKey::from(&self.signing_key);
        
        verifying_key.verify(payload, &signature)
            .map_err(|_| SovereignError::InvalidMissionFormat("Invalid signature".to_string()))
    }
    
    fn algorithm(&self) -> &'static str {
        "Ed25519"
    }
}

#[cfg(feature = "ed25519")]
impl Default for Ed25519Authority {
    fn default() -> Self {
        Self::new()
    }
}

/// Legacy CryptoAuthority - maintained for backward compatibility
pub struct CryptoAuthority {
    inner: HmacAuthority,
}

impl CryptoAuthority {
    pub fn new_hmac() -> SelResult<Self> {
        Ok(Self {
            inner: HmacAuthority::test_key(),
        })
    }
    
    pub fn sign(&self, canonical_mission: &str) -> SelResult<String> {
        self.inner.sign(canonical_mission.as_bytes())
    }
    
    pub fn verify(&self, canonical_mission: &str, signature: &str) -> SelResult<()> {
        self.inner.verify(canonical_mission.as_bytes(), signature)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_hmac_deterministic() {
        let auth1 = HmacAuthority::test_key();
        let auth2 = HmacAuthority::test_key();
        let msg = b"test message";
        
        let sig1 = auth1.sign(msg).unwrap();
        let sig2 = auth2.sign(msg).unwrap();
        
        assert_eq!(sig1, sig2);
    }
    
    #[test]
    fn test_legacy_crypto_authority() {
        let auth = CryptoAuthority::new_hmac().unwrap();
        let msg = "test mission";
        
        let sig = auth.sign(msg).unwrap();
        assert!(auth.verify(msg, &sig).is_ok());
    }
    
    #[cfg(feature = "ed25519")]
    #[test]
    fn test_ed25519() {
        let auth = Ed25519Authority::new();
        let msg = b"test message";
        
        let sig = auth.sign(msg).unwrap();
        assert!(auth.verify(msg, &sig).is_ok());
    }
}
