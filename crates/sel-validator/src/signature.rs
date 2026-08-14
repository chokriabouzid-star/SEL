//! # Signature Authority
//! SEL Core 1.1 — Ed25519 signing and independent verification
//!
//! ## Design
//! - `SignatureAuthority` trait: pluggable signature schemes
//! - `Ed25519Authority`: signs with a private key (validator side)
//! - `Ed25519Verifier`: verifies with a public key only (auditor side)
//!
//! The key insight: an auditor who holds only the *public* key and the
//! mission file can verify a proof without any shared secret — this is
//! what HMAC cannot provide (HMAC needs the same secret on both sides).

use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand::RngCore;
use sel_common::{SelResult, SovereignError};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

/// Core signature trait — pluggable signature schemes without breaking API.
pub trait SignatureAuthority {
    /// Sign `payload`, return signature as lowercase hex string.
    fn sign(&self, payload: &[u8]) -> SelResult<String>;

    /// Verify `signature_hex` against `payload`.
    fn verify(&self, payload: &[u8], signature_hex: &str) -> SelResult<()>;

    /// Algorithm identifier (e.g. `"ed25519"`).
    fn algorithm(&self) -> &'static str;
}

// ─────────────────────────────────────────────────────────────────────────────
// Ed25519Authority — signing side (holds private key)
// ─────────────────────────────────────────────────────────────────────────────

/// Ed25519 signing authority.
///
/// Key resolution priority (same pattern as `CryptoAuthority`):
/// 1. `SEL_ED25519_KEY_HEX` env var (64-char hex = 32-byte seed)
/// 2. Key persisted at `key_path` from a previous run
/// 3. Fresh random key → persisted to `key_path` (mode 0600)
pub struct Ed25519Authority {
    signing_key: SigningKey,
}

impl Ed25519Authority {
    /// Resolve key from env var → persisted file → generated random key.
    pub fn from_path_or_generate(key_path: &Path) -> Result<Self, String> {
        let env_hex = std::env::var("SEL_ED25519_KEY_HEX").ok();
        Self::_resolve(key_path, env_hex.as_deref())
    }

    /// Internal resolver — accepts env value as explicit parameter so tests
    /// can exercise every branch without touching process-global state.
    pub fn _resolve(key_path: &Path, env_hex: Option<&str>) -> Result<Self, String> {
        // Priority 1: env var
        if let Some(hex) = env_hex {
            let seed = hex::decode(hex.trim())
                .map_err(|e| format!("SEL_ED25519_KEY_HEX is not valid hex: {}", e))?;
            return Self::from_seed_bytes(&seed);
        }

        // Priority 2: persisted key
        if let Ok(bytes) = fs::read(key_path) {
            if let Ok(auth) = Self::from_seed_bytes(&bytes) {
                return Ok(auth);
            }
            // corrupt/wrong length — fall through and regenerate
        }

        // Priority 3: generate + persist
        let mut seed = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut seed);
        let signing_key = SigningKey::from_bytes(&seed);

        match Self::persist_key(key_path, &seed) {
            Ok(()) => eprintln!(
                "SEL WARNING: generated a new Ed25519 signing key → {}\n\
                 Back it up: losing it means past proofs cannot be re-signed.\n\
                 Pin a key permanently with SEL_ED25519_KEY_HEX.",
                key_path.display()
            ),
            Err(e) => eprintln!(
                "SEL WARNING: generated a new Ed25519 signing key but could \
                 not persist it to {} ({}).\n\
                 It will NOT survive a restart.",
                key_path.display(),
                e
            ),
        }

        Ok(Self { signing_key })
    }

    /// Build from a 32-byte seed.
    fn from_seed_bytes(seed: &[u8]) -> Result<Self, String> {
        let bytes: [u8; 32] = seed
            .try_into()
            .map_err(|_| format!("Ed25519 seed must be 32 bytes, got {}", seed.len()))?;
        Ok(Self {
            signing_key: SigningKey::from_bytes(&bytes),
        })
    }

    /// Default local path: `$HOME/.sel/ed25519.key`
    pub fn default_key_path() -> PathBuf {
        let base = std::env::var_os("HOME")
            .or_else(|| std::env::var_os("USERPROFILE"))
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("."));
        base.join(".sel").join("ed25519.key")
    }

    /// Hex-encoded public key — safe to publish; needed by `Ed25519Verifier`.
    pub fn public_key_hex(&self) -> String {
        hex::encode(self.signing_key.verifying_key().to_bytes())
    }

    /// Build an `Ed25519Verifier` from this authority's public key.
    /// Useful for self-verification in tests.
    pub fn to_verifier(&self) -> Ed25519Verifier {
        Ed25519Verifier {
            verifying_key: self.signing_key.verifying_key(),
        }
    }

    /// Build an `Ed25519Verifier` from a hex-encoded public key string.
    pub fn verifier_from_public_hex(public_key_hex: &str) -> Result<Ed25519Verifier, String> {
        Ed25519Verifier::from_public_hex(public_key_hex)
    }

    fn persist_key(key_path: &Path, seed: &[u8]) -> std::io::Result<()> {
        if let Some(parent) = key_path.parent() {
            fs::create_dir_all(parent)?;
        }

        // Create the file with mode 0600 atomically from the very first byte.
        // Using OpenOptionsExt::mode() sets the file permission at open(2) time,
        // eliminating the TOCTOU window that exists when File::create() (which
        // uses the process umask, typically 0644) is followed by set_permissions().
        // create_new(true) also prevents silently overwriting an existing key.
        #[cfg(unix)]
        let mut file = {
            use std::os::unix::fs::OpenOptionsExt;
            fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .mode(0o600)
                .open(key_path)?
        };

        #[cfg(not(unix))]
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(key_path)?;

        file.write_all(seed)?;
        file.sync_all()?;

        Ok(())
    }
}

impl SignatureAuthority for Ed25519Authority {
    fn sign(&self, payload: &[u8]) -> SelResult<String> {
        let signature: Signature = self.signing_key.sign(payload);
        Ok(hex::encode(signature.to_bytes()))
    }

    fn verify(&self, payload: &[u8], signature_hex: &str) -> SelResult<()> {
        self.to_verifier().verify(payload, signature_hex)
    }

    fn algorithm(&self) -> &'static str {
        "ed25519"
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Ed25519Verifier — verification side (public key only)
// ─────────────────────────────────────────────────────────────────────────────

/// Ed25519 verifier — holds only the **public** key.
///
/// This is what a third-party auditor uses: they receive the mission file,
/// the Ed25519 proof (hex), and the signer's public key (hex) — nothing
/// else. They can verify the proof without ever seeing the private key or
/// the HMAC secret.
pub struct Ed25519Verifier {
    verifying_key: VerifyingKey,
}

impl Ed25519Verifier {
    /// Build from a hex-encoded 32-byte public key.
    pub fn from_public_hex(hex_str: &str) -> Result<Self, String> {
        let bytes = hex::decode(hex_str.trim())
            .map_err(|e| format!("public key is not valid hex: {}", e))?;
        let arr: [u8; 32] = bytes
            .try_into()
            .map_err(|_| "Ed25519 public key must be 32 bytes".to_string())?;
        let verifying_key = VerifyingKey::from_bytes(&arr)
            .map_err(|e| format!("invalid Ed25519 public key: {}", e))?;
        Ok(Self { verifying_key })
    }

    /// Verify `signature_hex` against `payload`.
    /// Returns `Ok(())` on success, `Err(SovereignError)` on failure.
    pub fn verify(&self, payload: &[u8], signature_hex: &str) -> SelResult<()> {
        let sig_bytes = hex::decode(signature_hex.trim()).map_err(|e| {
            SovereignError::ValidationFailed(format!("signature is not valid hex: {}", e))
        })?;
        let sig_arr: [u8; 64] = sig_bytes.try_into().map_err(|_| {
            SovereignError::ValidationFailed("Ed25519 signature must be 64 bytes".to_string())
        })?;
        let signature = Signature::from_bytes(&sig_arr);

        self.verifying_key.verify(payload, &signature).map_err(|_| {
            SovereignError::ValidationFailed("Ed25519 signature verification failed".to_string())
        })
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_sign_and_verify_roundtrip() {
        let dir = tempdir().unwrap();
        let auth = Ed25519Authority::_resolve(&dir.path().join("k.key"), None).unwrap();
        let payload = b"hello SEL";
        let sig = auth.sign(payload).unwrap();
        assert!(auth.verify(payload, &sig).is_ok());
    }

    #[test]
    fn test_third_party_verifier_from_public_key() {
        // Simulate: signer produces proof, auditor verifies with public key only.
        let dir = tempdir().unwrap();
        let auth = Ed25519Authority::_resolve(&dir.path().join("k.key"), None).unwrap();
        let pubkey_hex = auth.public_key_hex();
        let payload = b"audit this mission";

        let sig = auth.sign(payload).unwrap();

        // Auditor side: only public key + payload + signature
        let verifier = Ed25519Authority::verifier_from_public_hex(&pubkey_hex).unwrap();
        assert!(verifier.verify(payload, &sig).is_ok());
    }

    #[test]
    fn test_tampered_payload_fails_verification() {
        let dir = tempdir().unwrap();
        let auth = Ed25519Authority::_resolve(&dir.path().join("k.key"), None).unwrap();
        let sig = auth.sign(b"original").unwrap();
        let verifier = auth.to_verifier();
        assert!(verifier.verify(b"tampered", &sig).is_err());
    }

    #[test]
    fn test_wrong_public_key_fails_verification() {
        let dir = tempdir().unwrap();
        let auth_a = Ed25519Authority::_resolve(&dir.path().join("a.key"), None).unwrap();
        let auth_b = Ed25519Authority::_resolve(&dir.path().join("b.key"), None).unwrap();
        let payload = b"payload";
        let sig = auth_a.sign(payload).unwrap();
        // Verifier built from auth_b's public key must reject auth_a's signature
        assert!(auth_b.to_verifier().verify(payload, &sig).is_err());
    }

    #[test]
    fn test_persisted_key_reused() {
        let dir = tempdir().unwrap();
        let key_path = dir.path().join("reuse.key");
        let auth1 = Ed25519Authority::_resolve(&key_path, None).unwrap();
        let auth2 = Ed25519Authority::_resolve(&key_path, None).unwrap();
        assert_eq!(auth1.public_key_hex(), auth2.public_key_hex());
    }

    #[test]
    fn test_env_var_key_used_when_present() {
        let seed = [0xABu8; 32];
        let seed_hex = hex::encode(seed);
        let dir = tempdir().unwrap();
        let auth =
            Ed25519Authority::_resolve(&dir.path().join("unused.key"), Some(&seed_hex)).unwrap();
        let auth2 =
            Ed25519Authority::_resolve(&dir.path().join("unused2.key"), Some(&seed_hex)).unwrap();
        // Same seed → same public key
        assert_eq!(auth.public_key_hex(), auth2.public_key_hex());
    }

    #[test]
    fn test_key_file_mode_600() {
        let dir = tempdir().unwrap();
        let key_path = dir.path().join("mode.key");
        Ed25519Authority::_resolve(&key_path, None).unwrap();

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mode = std::fs::metadata(&key_path).unwrap().permissions().mode() & 0o777;
            assert_eq!(mode, 0o600, "key file must be mode 0600, got {:o}", mode);
        }
    }
}
