//! # SEL Validator
//! Sovereign Validation Authority
//! SEL Core 1.0 Compliant

mod validator;
pub mod crypto_authority;  // 🔴 CHANGED: private → public
mod rules;
pub mod types;
pub mod signature;

pub use validator::{Validator, ValidationConfig};
pub use crypto_authority::{CryptoAuthority, HmacAuthority};
#[cfg(feature = "ed25519")]
pub use crypto_authority::Ed25519Authority;
pub use types::{
    ValidatedMission,
    ExecutionCapabilities,
    WorkspaceMode,
    ValidatedAction,
    ValidationProof,
};
pub use signature::SignatureAuthority;
pub use sel_common::SovereignError;

pub const VALIDATOR_VERSION: &str = "1.0.0";
