//! # SEL Common Library
//! SEL Core 1.0 - Shared Types and Utilities

pub mod canonical;
pub mod env_norm;
pub mod hash_chain;
pub mod errors;
pub mod resource_kind;

pub use canonical::{canonicalize_json, canonicalize_json_value};
pub use env_norm::normalize_environment;
pub use hash_chain::{HashChain, HashChainBuilder, GENESIS_HASH};
pub use errors::{SovereignError, SelResult};
pub use resource_kind::ResourceKind;

pub const SEL_VERSION: &str = "1.0.0";
pub const SEL_CORE_VERSION: &str = "1.0.0";
