//! # Cryptographic Authority
//! SEL Core 1.0 - HMAC-SHA256 Only
//! 🔴 NO Ed25519 in Core 1.0

use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

/// Cryptographic authority for Core 1.0 - HMAC only
pub struct CryptoAuthority {
    hmac_key: Vec<u8>,
}

impl CryptoAuthority {
    /// Create new CryptoAuthority with deterministic test key
    pub fn new() -> Self {
        Self {
            hmac_key: vec![
                0x53, 0x45, 0x4c, 0x5f, 0x43, 0x4f, 0x52, 0x45,
                0x5f, 0x31, 0x2e, 0x30, 0x5f, 0x4b, 0x45, 0x59,
            ],
        }
    }
    
    /// Sign with HMAC-SHA256 (deterministic)
    pub fn sign(&self, message: &str) -> String {
        let mut mac = HmacSha256::new_from_slice(&self.hmac_key)
            .expect("HMAC key length is valid");
        mac.update(message.as_bytes());
        let result = mac.finalize();
        hex::encode(result.into_bytes())
    }
    
    /// Verify HMAC signature
    pub fn verify(&self, message: &str, signature_hex: &str) -> bool {
        let expected = self.sign(message);
        expected == signature_hex
    }
}

impl Default for CryptoAuthority {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_hmac_deterministic() {
        let auth1 = CryptoAuthority::new();
        let auth2 = CryptoAuthority::new();
        let msg = "test message";
        
        let sig1 = auth1.sign(msg);
        let sig2 = auth2.sign(msg);
        
        assert_eq!(sig1, sig2);
        assert!(auth1.verify(msg, &sig1));
    }
}
