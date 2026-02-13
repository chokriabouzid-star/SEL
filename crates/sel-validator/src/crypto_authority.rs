//! # Cryptographic Authority
//! SEL Extended 1.1 - Dual Crypto Support
//! ✅ يعمل مع ed25519-dalek v2.2.0 و rand_core مع getrandom

use hmac::{Hmac, Mac};
use sha2::Sha256;
use serde::{Serialize, Deserialize};

type HmacSha256 = Hmac<Sha256>;

#[cfg(feature = "ed25519")]
use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Signer, Verifier};
#[cfg(feature = "ed25519")]
use rand_core::{OsRng, RngCore};

/// Type of cryptographic signature
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SignatureType {
    /// HMAC-SHA256 - deterministic, symmetric (Core 1.0)
    Hmac,
    /// Ed25519 - non-repudiation, asymmetric (Extended)
    #[cfg(feature = "ed25519")]
    Ed25519,
}

/// Cryptographic authority with dual-mode support
pub struct CryptoAuthority {
    hmac_key: Vec<u8>,
    #[cfg(feature = "ed25519")]
    ed25519_key: Option<SigningKey>,
    signature_type: SignatureType,
}

impl CryptoAuthority {
    /// Create new CryptoAuthority with HMAC only (Core 1.0)
    pub fn new_hmac() -> Self {
        Self {
            hmac_key: vec![
                0x53, 0x45, 0x4c, 0x5f, 0x43, 0x4f, 0x52, 0x45,
                0x5f, 0x31, 0x2e, 0x30, 0x5f, 0x4b, 0x45, 0x59,
            ],
            #[cfg(feature = "ed25519")]
            ed25519_key: None,
            signature_type: SignatureType::Hmac,
        }
    }
    
    /// Create new CryptoAuthority with Ed25519 (requires feature)
    #[cfg(feature = "ed25519")]
    pub fn new_ed25519() -> Self {
        // ✅ الطريقة الصحيحة لإنشاء SigningKey في v2.2.0
        let mut bytes = [0u8; 32];
        OsRng.fill_bytes(&mut bytes);
        let signing_key = SigningKey::from_bytes(&bytes);
        
        Self {
            hmac_key: Self::new_hmac().hmac_key,
            ed25519_key: Some(signing_key),
            signature_type: SignatureType::Ed25519,
        }
    }
    
    /// Create with custom HMAC key
    pub fn with_hmac_key(key: Vec<u8>) -> Self {
        Self {
            hmac_key: key,
            #[cfg(feature = "ed25519")]
            ed25519_key: None,
            signature_type: SignatureType::Hmac,
        }
    }
    
    /// Get current signature type
    pub fn signature_type(&self) -> SignatureType {
        self.signature_type
    }
    
    /// Sign a message with current crypto
    pub fn sign(&self, message: &str) -> String {
        match self.signature_type {
            SignatureType::Hmac => self.sign_hmac(message),
            #[cfg(feature = "ed25519")]
            SignatureType::Ed25519 => self.sign_ed25519(message).unwrap_or_else(|| self.sign_hmac(message)),
        }
    }
    
    /// Sign with HMAC-SHA256 (deterministic)
    fn sign_hmac(&self, message: &str) -> String {
        let mut mac = HmacSha256::new_from_slice(&self.hmac_key)
            .expect("HMAC key length is valid");
        mac.update(message.as_bytes());
        let result = mac.finalize();
        hex::encode(result.into_bytes())
    }
    
    /// Sign with Ed25519 (non-repudiation)
    #[cfg(feature = "ed25519")]
    fn sign_ed25519(&self, message: &str) -> Option<String> {
        self.ed25519_key.as_ref().map(|key| {
            let signature: Signature = key.sign(message.as_bytes());
            hex::encode(signature.to_bytes())
        })
    }
    
    /// Verify a signature
    pub fn verify(&self, message: &str, signature_hex: &str, sig_type: SignatureType) -> bool {
        match sig_type {
            SignatureType::Hmac => self.verify_hmac(message, signature_hex),
            #[cfg(feature = "ed25519")]
            SignatureType::Ed25519 => self.verify_ed25519(message, signature_hex),
        }
    }
    
    /// Verify HMAC signature
    fn verify_hmac(&self, message: &str, signature_hex: &str) -> bool {
        let expected = self.sign_hmac(message);
        expected == signature_hex
    }
    
    /// Verify Ed25519 signature
    #[cfg(feature = "ed25519")]
    fn verify_ed25519(&self, message: &str, signature_hex: &str) -> bool {
        if let Some(key) = &self.ed25519_key {
            let verifying_key = VerifyingKey::from(key);
            let signature_bytes = match hex::decode(signature_hex) {
                Ok(bytes) => bytes,
                Err(_) => return false,
            };
            
            if signature_bytes.len() != 64 {
                return false;
            }
            let mut array = [0u8; 64];
            array.copy_from_slice(&signature_bytes);
            
            // ✅ في v2.2.0، from_bytes تعمل مباشرة
            let signature = Signature::from_bytes(&array);
            
            verifying_key.verify(message.as_bytes(), &signature).is_ok()
        } else {
            false
        }
    }
    
    /// Get public key (Ed25519)
    #[cfg(feature = "ed25519")]
    pub fn public_key(&self) -> Option<String> {
        self.ed25519_key.as_ref().map(|key| {
            let verifying_key = VerifyingKey::from(key);
            hex::encode(verifying_key.to_bytes())
        })
    }
    
    /// Get HMAC key (for verification only)
    pub fn hmac_key(&self) -> &[u8] {
        &self.hmac_key
    }
}

impl Default for CryptoAuthority {
    fn default() -> Self {
        Self::new_hmac()  // Core 1.0 compatibility
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_hmac_deterministic() {
        let auth1 = CryptoAuthority::new_hmac();
        let auth2 = CryptoAuthority::new_hmac();
        let msg = "test message";
        
        let sig1 = auth1.sign(msg);
        let sig2 = auth2.sign(msg);
        
        assert_eq!(sig1, sig2);
        assert!(auth1.verify(msg, &sig1, SignatureType::Hmac));
    }
    
    #[cfg(feature = "ed25519")]
    #[test]
    fn test_ed25519_sign_verify() {
        let auth = CryptoAuthority::new_ed25519();
        let msg = "test message";
        
        let sig = auth.sign(msg);
        assert!(auth.verify(msg, &sig, SignatureType::Ed25519));
        
        // Public key should exist
        assert!(auth.public_key().is_some());
    }
    
    #[cfg(feature = "ed25519")]
    #[test]
    fn test_dual_crypto_compatibility() {
        let auth_hmac = CryptoAuthority::new_hmac();
        let auth_ed = CryptoAuthority::new_ed25519();
        let msg = "test message";
        
        let sig_hmac = auth_hmac.sign(msg);
        let sig_ed = auth_ed.sign(msg);
        
        assert_ne!(sig_hmac, sig_ed);
        
        assert!(auth_hmac.verify(msg, &sig_hmac, SignatureType::Hmac));
        assert!(auth_ed.verify(msg, &sig_ed, SignatureType::Ed25519));
        
        assert!(!auth_hmac.verify(msg, &sig_ed, SignatureType::Hmac));
        assert!(!auth_ed.verify(msg, &sig_hmac, SignatureType::Ed25519));
    }
}
