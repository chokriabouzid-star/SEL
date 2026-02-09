//! SEL Validator - Sovereign Validation Gateway

pub mod types;
pub mod validator;

// Re-exports
pub use types::{
    ValidatedMission,
    ExecutionCapabilities,
    WorkspaceMode,
    SemanticVersion,
    VALIDATOR_VERSION,
    ProofError,
};

pub use validator::{
    Validator,
    ValidationResult,
    ValidationError,
    ErrorType,
};
