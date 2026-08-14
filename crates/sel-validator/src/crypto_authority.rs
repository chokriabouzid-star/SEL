//! # Cryptographic Authority
//! SEL Core 1.0 — HMAC-SHA256
//!
//! Key resolution priority (production path):
//!   1. `SEL_HMAC_KEY_HEX` environment variable (hex-encoded, ≥16 bytes)
//!   2. Key persisted at `key_path` from a previous run
//!   3. Fresh 32-byte random key → persisted to `key_path` (mode 0600)
//!
//! The fixed "test key" (`insecure_fixed_test_key`) is compiled in only
//! under `#[cfg(test)]` and is therefore unreachable from any release
//! binary, the CLI, or downstream crates.

use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

type HmacSha256 = Hmac<Sha256>;

/// Minimum accepted key length in bytes.
const MIN_KEY_BYTES: usize = 16;

/// Cryptographic authority for Core 1.0 — HMAC-SHA256 only.
pub struct CryptoAuthority {
    hmac_key: Vec<u8>,
}

impl CryptoAuthority {
    // ------------------------------------------------------------------ //
    //  Test-only constructor (not compiled into release builds)           //
    // ------------------------------------------------------------------ //

    /// ⚠️  INSECURE — fixed, publicly-known key (visible in this
    /// open-source file).  Unit-test / determinism-check use ONLY.
    /// Not callable from production code: `#[cfg(test)]` removes it
    /// from every release build at the compiler level.
    #[cfg(test)]
    fn insecure_fixed_test_key() -> Self {
        Self {
            // "SEL_CORE_1.0_KEY" in ASCII
            hmac_key: vec![
                0x53, 0x45, 0x4c, 0x5f, 0x43, 0x4f, 0x52, 0x45, 0x5f, 0x31, 0x2e, 0x30, 0x5f, 0x4b,
                0x45, 0x59,
            ],
        }
    }

    // ------------------------------------------------------------------ //
    //  Production constructors                                            //
    // ------------------------------------------------------------------ //

    /// Build from an explicit key supplied by the caller (e.g. from a KMS
    /// or a secret store).  Rejects keys shorter than `MIN_KEY_BYTES` to
    /// prevent trivially brute-forceable HMAC outputs.
    pub fn from_key(key: Vec<u8>) -> Result<Self, String> {
        if key.len() < MIN_KEY_BYTES {
            return Err(format!(
                "HMAC key too short: {} bytes (minimum {})",
                key.len(),
                MIN_KEY_BYTES
            ));
        }
        Ok(Self { hmac_key: key })
    }

    /// Production key resolution — used by `Validator::new()` and the CLI.
    ///
    /// Reads `SEL_HMAC_KEY_HEX` from the real process environment, then
    /// delegates to `_resolve`. Tests that need to control the env-var
    /// value call `_resolve` directly with an explicit parameter so they
    /// never touch process-global state (which is shared across parallel
    /// test threads and causes race conditions).
    pub fn from_env_or_generate(key_path: &Path) -> Result<Self, String> {
        let env_hex = std::env::var("SEL_HMAC_KEY_HEX").ok();
        Self::_resolve(key_path, env_hex.as_deref())
    }

    /// Internal resolver — accepts the env-var value as an explicit
    /// parameter so every priority branch is testable without mutating
    /// process-global state.
    ///
    /// Priority:
    /// 1. `env_hex` is `Some(hex_string)` — operator-supplied key.
    /// 2. Key previously persisted at `key_path`.
    /// 3. Fresh 32-byte random key persisted to `key_path` (mode 0600).
    ///
    /// A malformed hex string in `env_hex` is an immediate hard error.
    pub fn _resolve(key_path: &Path, env_hex: Option<&str>) -> Result<Self, String> {
        // --- Priority 1: explicit env var ---
        if let Some(hex_key) = env_hex {
            let key = hex::decode(hex_key.trim())
                .map_err(|e| format!("SEL_HMAC_KEY_HEX is not valid hex: {}", e))?;
            return Self::from_key(key);
        }

        // --- Priority 2: persisted key from a previous run ---
        if let Ok(existing) = fs::read(key_path) {
            if let Ok(auth) = Self::from_key(existing) {
                return Ok(auth);
            }
            // File exists but is corrupt / too short — regenerate below.
        }

        // --- Priority 3: generate a new random key and persist it ---
        use rand::RngCore;
        let mut key = vec![0u8; 32];
        rand::thread_rng().fill_bytes(&mut key);

        match Self::persist_key(key_path, &key) {
            Ok(()) => eprintln!(
                "SEL WARNING: generated a new local HMAC signing key → {}\n\
                 Back it up: losing it invalidates all proofs issued under it.\n\
                 Pin a key permanently with SEL_HMAC_KEY_HEX.",
                key_path.display()
            ),
            Err(e) => eprintln!(
                "SEL WARNING: generated a new HMAC signing key but could not \
                 persist it to {} ({}).\n\
                 It will NOT survive a restart — set SEL_HMAC_KEY_HEX to pin \
                 a key explicitly.",
                key_path.display(),
                e
            ),
        }

        Self::from_key(key)
    }

    /// Default local path for the persisted key:
    /// `$HOME/.sel/hmac.key`  (falls back to `.` if HOME is unset).
    pub fn default_key_path() -> PathBuf {
        let base = std::env::var_os("HOME")
            .or_else(|| std::env::var_os("USERPROFILE"))
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("."));
        base.join(".sel").join("hmac.key")
    }

    // ------------------------------------------------------------------ //
    //  Private helpers                                                    //
    // ------------------------------------------------------------------ //

    fn persist_key(key_path: &Path, key: &[u8]) -> std::io::Result<()> {
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

        // On non-Unix platforms create_new still prevents clobbering;
        // file-system ACLs are the operator's responsibility there.
        #[cfg(not(unix))]
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(key_path)?;

        file.write_all(key)?;
        // sync_all() ensures the key bytes reach physical storage before we
        // return — so a crash immediately after persist_key() cannot leave a
        // zero-byte or partial key file that a future run silently accepts.
        file.sync_all()?;

        Ok(())
    }

    // ------------------------------------------------------------------ //
    //  Core cryptographic operations                                      //
    // ------------------------------------------------------------------ //

    /// Sign `message` with HMAC-SHA256.  Output is deterministic for a
    /// fixed key (used directly by the hash-chain / proof logic).
    pub fn sign(&self, message: &str) -> String {
        let mut mac = HmacSha256::new_from_slice(&self.hmac_key).expect("HMAC key length is valid");
        mac.update(message.as_bytes());
        hex::encode(mac.finalize().into_bytes())
    }

    /// Verify a previously produced HMAC signature.
    pub fn verify(&self, message: &str, signature_hex: &str) -> bool {
        self.sign(message) == signature_hex
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    // ---- existing test (key renamed, behaviour identical) ----

    #[test]
    fn test_hmac_deterministic() {
        let auth1 = CryptoAuthority::insecure_fixed_test_key();
        let auth2 = CryptoAuthority::insecure_fixed_test_key();
        let msg = "test message";
        let sig1 = auth1.sign(msg);
        let sig2 = auth2.sign(msg);
        assert_eq!(sig1, sig2);
        assert!(auth1.verify(msg, &sig1));
    }

    // ---- new tests for production key resolution ----

    #[test]
    fn test_from_key_rejects_short_key() {
        assert!(CryptoAuthority::from_key(vec![1, 2, 3]).is_err());
    }

    #[test]
    fn test_from_key_accepts_valid_key() {
        assert!(CryptoAuthority::from_key(vec![7u8; 32]).is_ok());
    }

    #[test]
    fn test_env_var_key_is_used_when_present() {
        // Uses _resolve directly with an explicit hex value —
        // zero process-global state mutation, safe under parallel execution.
        let key_bytes = vec![9u8; 32];
        let key_hex = hex::encode(&key_bytes);
        let dir = tempdir().unwrap();

        let auth =
            CryptoAuthority::_resolve(&dir.path().join("unused.key"), Some(&key_hex)).unwrap();

        let expected = CryptoAuthority::from_key(key_bytes).unwrap();
        assert_eq!(auth.sign("msg"), expected.sign("msg"));
    }

    #[test]
    fn test_different_key_paths_produce_different_keys() {
        // Passes None explicitly → exercises random-generation path only,
        // with zero dependency on process-global env-var state.
        // Two independent key paths must never produce the same key.
        let dir = tempdir().unwrap();
        let auth_a = CryptoAuthority::_resolve(&dir.path().join("a.key"), None).unwrap();
        let auth_b = CryptoAuthority::_resolve(&dir.path().join("b.key"), None).unwrap();
        assert_ne!(auth_a.sign("msg"), auth_b.sign("msg"));
    }

    #[test]
    fn test_persisted_key_reused_across_instances() {
        // Same path → same key loaded from disk → identical signatures.
        let dir = tempdir().unwrap();
        let key_path = dir.path().join("reuse.key");
        let auth1 = CryptoAuthority::_resolve(&key_path, None).unwrap();
        let auth2 = CryptoAuthority::_resolve(&key_path, None).unwrap();
        assert_eq!(auth1.sign("msg"), auth2.sign("msg"));
    }

    #[test]
    fn test_persisted_key_file_mode_600() {
        let dir = tempdir().unwrap();
        let key_path = dir.path().join("mode.key");
        CryptoAuthority::_resolve(&key_path, None).unwrap();

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let meta = std::fs::metadata(&key_path).unwrap();
            let mode = meta.permissions().mode() & 0o777;
            assert_eq!(mode, 0o600, "key file must be mode 0600, got {:o}", mode);
        }
    }
}
