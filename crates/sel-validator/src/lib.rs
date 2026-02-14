//! # SEL Validator
//! SEL Core 1.0 - HMAC only

mod validator;
mod crypto_authority;
mod rules;
pub mod types;

pub use validator::{Validator, ValidationConfig};
pub use crypto_authority::CryptoAuthority;
pub use types::{
    ValidatedMission,
    ExecutionCapabilities,
    WorkspaceMode,
    ValidatedAction,
    ValidationProof,
};
pub use sel_common::SovereignError;

pub const VALIDATOR_VERSION: &str = "1.0.0";
