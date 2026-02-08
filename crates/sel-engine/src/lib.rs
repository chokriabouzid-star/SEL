//! SEL Engine Library
//!
//! Sovereign Execution Layer - Deterministic execution engine

pub mod canonical_adapter;
pub mod engine;

// Re-export main types
pub use engine::{FactsLogger, MissionExecutor};
pub use canonical_adapter::{canonicalize_mission, create_hash_chain};

/// Result type for SEL operations
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
