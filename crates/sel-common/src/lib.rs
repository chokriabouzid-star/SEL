//! # SEL Common Library
//! SEL Extended 1.1.0-alpha - Fully Deterministic, No Panics

pub mod canonical;
pub mod env_norm;
pub mod errors;
pub mod hash_chain;
pub mod resource_kind;

pub use canonical::{canonicalize_json, canonicalize_json_value};
pub use env_norm::{is_likely_path, normalize_environment, normalize_path_field};
pub use errors::{SelResult, SovereignError};
pub use hash_chain::{HashChain, HashChainBuilder, GENESIS_HASH};
pub use resource_kind::ResourceKind;

/// SEL version — derived from Cargo.toml at compile time so it can never
/// drift from the actual package version. This value is written into every
/// ValidatedMission's `core_version` field and therefore into every proof.
pub const SEL_VERSION: &str = env!("CARGO_PKG_VERSION");
