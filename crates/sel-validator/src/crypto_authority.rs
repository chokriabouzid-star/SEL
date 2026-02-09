//! Cryptographic Authority - Single Source of Truth for Proof Generation
//!
//! 🔴 SOVEREIGN AUTHORITY: This module owns the ONE AND ONLY truth
//! for validation proof generation and verification.

use serde_json::Value;
use sha2::{Sha256, Digest};
use hmac::{Hmac, Mac};
use hex;

use crate::types::{
    SemanticVersion, ExecutionCapabilities, WorkspaceMode,
    ProofError,
};

type HmacSha256 = Hmac<Sha256>;

/// Create proof payload (canonical + deterministic)
/// NON-NEGOTIABLE format: mission_hash || validator_version || capabilities || workspace_mode
///
/// 🔴 AUTHORITY FUNCTION: Only this function creates payloads
/// 🔴 SINGLE SOURCE OF TRUTH: No other function may create payloads
pub fn create_proof_payload(
    mission: &Value,
    version: &SemanticVersion,
    capabilities: &ExecutionCapabilities,
    workspace_mode: WorkspaceMode,
) -> Vec<u8> {
    // 1. Mission hash (SHA256 of canonical JSON)
    let mission_json = serde_json::to_string(mission)
        .expect("Failed to serialize mission");
    let mut hasher = Sha256::new();
    hasher.update(mission_json.as_bytes());
    let mission_hash = format!("{:x}", hasher.finalize());

    // 2. Capabilities as canonical JSON
    let capabilities_json = serde_json::to_string(capabilities)
        .expect("Failed to serialize capabilities");

    // 3. Workspace mode as string
    let workspace_mode_str = match workspace_mode {
        WorkspaceMode::ReadOnly => "ReadOnly",
        WorkspaceMode::ReadWrite => "ReadWrite",
    };

    // 4. Concatenate as per specification
    let payload_str = format!(
        "{}||{}||{}||{}",
        mission_hash,
        version,
        capabilities_json,
        workspace_mode_str
    );

    payload_str.into_bytes()
}

/// Compute validation proof using HMAC-SHA256
/// 
/// 🔴 AUTHORITY FUNCTION: Only this function signs payloads
/// 🔴 SINGLE SOURCE OF TRUTH: No other function may sign
pub fn compute_validation_proof(payload: &[u8], secret_key: &[u8]) -> String {
    // Create HMAC instance
    let mut mac = HmacSha256::new_from_slice(secret_key)
        .expect("HMAC can take key of any size");
    
    // Update with payload
    mac.update(payload);
    
    // Finalize and encode
    let result = mac.finalize();
    let code_bytes = result.into_bytes();
    
    format!("hmac-sha256:{}", hex::encode(code_bytes))
}

/// Verify validation proof using HMAC-SHA256
/// 
/// 🔴 AUTHORITY FUNCTION: Only this function verifies proofs
/// 🔴 SINGLE SOURCE OF TRUTH: No other function may verify
pub fn verify_validation_proof(
    payload: &[u8],
    secret_key: &[u8],
    provided_proof: &str,
) -> Result<(), ProofError> {
    let expected = compute_validation_proof(payload, secret_key);
    
    if expected != provided_proof {
        return Err(ProofError::InvalidSignature {
            expected,
            found: provided_proof.to_string(),
        });
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use crate::types::VALIDATOR_VERSION;

    #[test]
    fn test_authority_single_source_of_truth() {
        let mission = json!({"name": "test"});
        let version = VALIDATOR_VERSION.clone();
        let caps = ExecutionCapabilities::default();
        let mode = WorkspaceMode::ReadOnly;
        let secret_key = b"test-secret-key-32-bytes-long-here!";
        
        // ONLY this function creates payloads
        let payload = create_proof_payload(&mission, &version, &caps, mode);
        
        // ONLY this function signs
        let proof1 = compute_validation_proof(&payload, secret_key);
        
        // ONLY this function verifies
        let result = verify_validation_proof(&payload, secret_key, &proof1);
        assert!(result.is_ok());
        
        // Proof is deterministic
        let proof2 = compute_validation_proof(&payload, secret_key);
        assert_eq!(proof1, proof2);
    }
    
    #[test]
    fn test_payload_deterministic() {
        let mission = json!({"name": "test"});
        let version = VALIDATOR_VERSION.clone();
        let caps = ExecutionCapabilities::default();
        let mode = WorkspaceMode::ReadOnly;
        
        let payload1 = create_proof_payload(&mission, &version, &caps, mode);
        let payload2 = create_proof_payload(&mission, &version, &caps, mode);
        
        assert_eq!(payload1, payload2, "Payload must be deterministic");
    }
}
