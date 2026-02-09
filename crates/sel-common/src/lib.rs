//! SEL Common - Shared types and utilities

/// Common error type
#[derive(Debug, thiserror::Error)]
pub enum SelError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Execution error: {0}")]
    Execution(String),
}

/// Result type for SEL operations
pub type Result<T> = std::result::Result<T, SelError>;
