//! SEL Engine - Sovereign Execution Layer
//! 
//! Deterministic execution engine with canonicalization support

pub mod engine;
pub mod mission;

// Canonicalization support
pub mod canonical_adapter;
pub use canonical_adapter::{
    canonicalize_mission, 
    create_hash_chain, 
    format_mission_hash,
};

/// Result type for SEL operations
pub type Result<T> = std::result::Result<T, Error>;

/// Error type for SEL operations
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Execution error: {0}")]
    Execution(String),
    
    #[error("Canonicalization error: {0}")]
    Canonicalization(String),
}
