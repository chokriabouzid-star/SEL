//! # SEL Validator
//! SEL Core 1.0 - HMAC only

mod crypto_authority;
mod rules;
pub mod types;
mod validator;

pub use crypto_authority::CryptoAuthority;
pub use sel_common::SovereignError;
pub use types::{
    ExecutionCapabilities, ValidatedAction, ValidatedMission, ValidationProof, WorkspaceMode,
};
pub use validator::{ValidationConfig, Validator};

pub const VALIDATOR_VERSION: &str = "1.0.0";
