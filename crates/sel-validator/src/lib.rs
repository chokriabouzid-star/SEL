//! # SEL Validator
//! SEL Extended 1.1 - Dual Crypto Support (HMAC + Ed25519)

mod validator;
mod crypto_authority;
mod rules;
pub mod types;

pub use validator::{Validator, ValidationConfig};
pub use crypto_authority::{CryptoAuthority, SignatureType};
pub use types::{
    ValidatedMission,
    ExecutionCapabilities,
    WorkspaceMode,
    ValidatedAction,
    ValidationProof,
};
pub use sel_common::SovereignError;

pub const VALIDATOR_VERSION: &str = "1.1.0-alpha";
