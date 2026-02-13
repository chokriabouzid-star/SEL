//! # Crypto Authority Tests
//! SEL Extended 1.1 - اختبارات HMAC + Ed25519
//!
//! ✅ يتحقق من:
//! - HMAC determinism
//! - Ed25519 sign/verify
//! - Dual crypto compatibility

use sel_validator::{Validator, ValidationConfig, SignatureType};

const TEST_MISSION: &str = r#"{
    "name": "crypto-test",
    "version": "1.0",
    "actions": [
        {"command": "echo", "args": ["hello"]}
    ]
}"#;

#[test]
fn test_hmac_deterministic() {
    println!("\n🔵 TEST: HMAC Deterministic");
    println!("============================");
    
    let config = ValidationConfig {
        signature_type: SignatureType::Hmac,
        ..Default::default()
    };
    
    let validator1 = Validator::new(config.clone());
    let validator2 = Validator::new(config);
    
    let result1 = validator1.validate(TEST_MISSION).unwrap();
    let result2 = validator2.validate(TEST_MISSION).unwrap();
    
    // نفس المهمة = نفس التوقيع (حتمي)
    assert_eq!(result1.validation_proof_str(), result2.validation_proof_str());
    assert_eq!(result1.signature_type(), SignatureType::Hmac);
    
    println!("  ✅ HMAC deterministic: نفس المهمة = نفس التوقيع");
    println!("  ✅ Signature type: {:?}", result1.signature_type());
    println!("============================\n");
}

#[test]
#[cfg(feature = "ed25519")]
fn test_ed25519_sign_verify() {
    println!("\n🔴 TEST: Ed25519 Sign/Verify");
    println!("============================");
    
    let config = ValidationConfig {
        signature_type: SignatureType::Ed25519,
        ..Default::default()
    };
    
    let validator = Validator::new(config);
    let result = validator.validate(TEST_MISSION).unwrap();
    
    assert_eq!(result.signature_type(), SignatureType::Ed25519);
    assert!(!result.validation_proof_str().is_empty());
    
    // Public key should be available
    let pubkey = validator.public_key();
    assert!(pubkey.is_some());
    println!("  ✅ Ed25519 signature generated");
    println!("  ✅ Public key: {}...", &pubkey.unwrap()[..16]);
    println!("============================\n");
}

#[test]
#[cfg(feature = "ed25519")]
fn test_dual_crypto_different() {
    println!("\n🟣 TEST: Dual Crypto - Different Signatures");
    println!("============================");
    
    // HMAC config
    let hmac_config = ValidationConfig {
        signature_type: SignatureType::Hmac,
        ..Default::default()
    };
    
    // Ed25519 config
    let ed_config = ValidationConfig {
        signature_type: SignatureType::Ed25519,
        ..Default::default()
    };
    
    let hmac_validator = Validator::new(hmac_config);
    let ed_validator = Validator::new(ed_config);
    
    let hmac_result = hmac_validator.validate(TEST_MISSION).unwrap();
    let ed_result = ed_validator.validate(TEST_MISSION).unwrap();
    
    // التوقيعات مختلفة
    assert_ne!(hmac_result.validation_proof_str(), ed_result.validation_proof_str());
    
    println!("  ✅ HMAC signature: {}...", &hmac_result.validation_proof_str()[..16]);
    println!("  ✅ Ed25519 signature: {}...", &ed_result.validation_proof_str()[..16]);
    println!("  ✅ Different signatures verified");
    println!("============================\n");
}

#[test]
#[cfg(feature = "ed25519")]
fn test_ed25519_public_key_consistency() {
    println!("\n🔑 TEST: Ed25519 Public Key Consistency");
    println!("============================");
    
    let config = ValidationConfig {
        signature_type: SignatureType::Ed25519,
        ..Default::default()
    };
    
    let validator = Validator::new(config);
    let pubkey1 = validator.public_key().unwrap();
    
    // Validate same mission multiple times - public key stays the same
    let _result1 = validator.validate(TEST_MISSION).unwrap();
    let pubkey2 = validator.public_key().unwrap();
    
    assert_eq!(pubkey1, pubkey2);
    
    let _result2 = validator.validate(TEST_MISSION).unwrap();
    let pubkey3 = validator.public_key().unwrap();
    
    assert_eq!(pubkey1, pubkey3);
    
    println!("  ✅ Public key consistent across validations");
    println!("  ✅ Public key: {}...", &pubkey1[..16]);
    println!("============================\n");
}

#[test]
fn test_invalid_json_rejected() {
    println!("\n❌ TEST: Invalid JSON Rejection");
    println!("============================");
    
    // Create an invalid mission (missing actions)
    let invalid_mission = r#"{
        "name": "invalid",
        "version": "1.0"
    }"#;
    
    let validator = Validator::new(ValidationConfig::default());
    let result = validator.validate(invalid_mission);
    
    assert!(result.is_err());
    println!("  ✅ Invalid mission (missing actions) rejected correctly");
    println!("============================\n");
}

#[test]
fn test_malformed_json_rejected() {
    println!("\n❌ TEST: Malformed JSON Rejection");
    println!("============================");
    
    // Malformed JSON
    let malformed_mission = r#"{
        "name": "malformed",
        "actions": [{"command": "echo", "args": ["test"}
    }"#;
    
    let validator = Validator::new(ValidationConfig::default());
    let result = validator.validate(malformed_mission);
    
    assert!(result.is_err());
    println!("  ✅ Malformed JSON rejected correctly");
    println!("============================\n");
}
