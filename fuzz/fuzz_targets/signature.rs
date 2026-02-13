#![no_main]

use libfuzzer_sys::fuzz_target;
use sel_validator::HmacAuthority;
use sel_validator::SignatureAuthority;  // ✅ IMPORTANT: استيراد التريت
use sel_common::SovereignError;

fuzz_target!(|data: &[u8]| {
    // Test HMAC with fixed key (deterministic)
    let auth = HmacAuthority::test_key();
    
    // ✅ PROPERTY 1: Sign + Verify is idempotent
    if let Ok(sig) = auth.sign(data) {
        assert!(
            auth.verify(data, &sig).is_ok(),
            "Signature verification failed for valid signature"
        );
        
        // ✅ PROPERTY 2: Different payloads have different signatures
        if !data.is_empty() {
            let mut different = data.to_vec();
            different[0] ^= 0xff;
            
            if let Ok(sig2) = auth.sign(&different) {
                assert_ne!(
                    sig, sig2,
                    "Different payloads produced same signature"
                );
            }
        }
    }
    
    // ✅ إنشاء strings خارج الـ vector لمنع مشكلة التوقيت
    let zero_repeated = "0".repeat(128);
    let ff_repeated = "ff".repeat(32);
    
    // ✅ PROPERTY 3: Invalid signatures are rejected
    let invalid_sigs: Vec<&str> = vec![
        "",
        "invalid",
        "deadbeef",
        &zero_repeated,  // ✅ الآن reference صالح
        &ff_repeated,    // ✅ الآن reference صالح
    ];
    
    for invalid in invalid_sigs {
        assert!(
            auth.verify(data, invalid).is_err(),
            "Invalid signature was accepted: {}",
            invalid
        );
    }
    
    // ✅ PROPERTY 4: Empty payload is handled gracefully
    let empty: &[u8] = &[];
    if let Ok(sig) = auth.sign(empty) {
        assert!(auth.verify(empty, &sig).is_ok());
    }
});
