//! # SEL Common Library
//! SEL Extended 1.1.0-alpha - Fully Deterministic, No Panics

pub mod canonical;
pub mod env_norm;
pub mod hash_chain;
pub mod errors;
pub mod resource_kind;

pub use canonical::{canonicalize_json, canonicalize_json_value};
pub use env_norm::{normalize_environment, normalize_path_field, is_likely_path};
pub use hash_chain::{HashChain, HashChainBuilder, GENESIS_HASH};
pub use errors::{SovereignError, SelResult};
pub use resource_kind::ResourceKind;

pub const SEL_VERSION: &str = "1.1.0-alpha";
pub const SEL_CORE_VERSION: &str = "1.0.0";
pub const SEL_EXTENDED_VERSION: &str = "1.1.0-alpha";
