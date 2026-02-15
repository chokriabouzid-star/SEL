//! # Sovereign Error Model
//! SEL Core 1.0 - Official Error Enumeration

use crate::ResourceKind;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SovereignError {
    // ===== 1xx - Validation Errors =====
    ValidationFailed(String),     // 101
    InvalidMissionFormat(String), // 102
    MissingValidationProof,       // 103

    // ===== 2xx - Workspace Errors =====
    WorkspaceCreationFailed(String), // 201
    WorkspaceViolation(String),      // 202
    WorkspaceCleanupFailed(String),  // 203

    // ===== 3xx - Capability Errors =====
    CapabilityViolation(String),      // 301
    InsufficientCapabilities(String), // 302

    // ===== 4xx - Resource Errors =====
    ResourceExhaustion {
        kind: ResourceKind,
        limit: u64,
        requested: u64,
    }, // 401

    // ===== 5xx - Determinism Errors =====
    DeterminismViolation(String), // 501
    HashMismatch {
        expected: String,
        actual: String,
    }, // 502
    /// 🔴 NEW: Floating point numbers are non-deterministic
    NonDeterministicNumber, // 503

    // ===== 9xx - Internal Errors =====
    InternalError(String), // 901
}

impl SovereignError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::ValidationFailed(_) => "101",
            Self::InvalidMissionFormat(_) => "102",
            Self::MissingValidationProof => "103",
            Self::WorkspaceCreationFailed(_) => "201",
            Self::WorkspaceViolation(_) => "202",
            Self::WorkspaceCleanupFailed(_) => "203",
            Self::CapabilityViolation(_) => "301",
            Self::InsufficientCapabilities(_) => "302",
            Self::ResourceExhaustion { .. } => "401",
            Self::DeterminismViolation(_) => "501",
            Self::HashMismatch { .. } => "502",
            Self::NonDeterministicNumber => "503",
            Self::InternalError(_) => "901",
        }
    }
}

impl fmt::Display for SovereignError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ValidationFailed(msg) => write!(f, "[SEL-101] Validation failed: {}", msg),
            Self::InvalidMissionFormat(msg) => {
                write!(f, "[SEL-102] Invalid mission format: {}", msg)
            }
            Self::MissingValidationProof => write!(f, "[SEL-103] Missing validation proof"),
            Self::WorkspaceCreationFailed(msg) => {
                write!(f, "[SEL-201] Workspace creation failed: {}", msg)
            }
            Self::WorkspaceViolation(msg) => write!(f, "[SEL-202] Workspace violation: {}", msg),
            Self::WorkspaceCleanupFailed(msg) => {
                write!(f, "[SEL-203] Workspace cleanup failed: {}", msg)
            }
            Self::CapabilityViolation(msg) => write!(f, "[SEL-301] Capability violation: {}", msg),
            Self::InsufficientCapabilities(msg) => {
                write!(f, "[SEL-302] Insufficient capabilities: {}", msg)
            }
            Self::ResourceExhaustion {
                kind,
                limit,
                requested,
            } => write!(
                f,
                "[SEL-401] Resource exhaustion: {} (limit={}, requested={})",
                kind, limit, requested
            ),
            Self::DeterminismViolation(msg) => {
                write!(f, "[SEL-501] Determinism violation: {}", msg)
            }
            Self::HashMismatch { expected, actual } => write!(
                f,
                "[SEL-502] Hash mismatch (expected={}, actual={})",
                expected, actual
            ),
            Self::NonDeterministicNumber => write!(
                f,
                "[SEL-503] Non-deterministic number: floating point or big integer"
            ),
            Self::InternalError(msg) => write!(f, "[SEL-901] Internal error: {}", msg),
        }
    }
}

impl std::error::Error for SovereignError {}

impl From<serde_json::Error> for SovereignError {
    fn from(err: serde_json::Error) -> Self {
        SovereignError::InvalidMissionFormat(format!("JSON serialization error: {}", err))
    }
}

impl From<std::io::Error> for SovereignError {
    fn from(err: std::io::Error) -> Self {
        SovereignError::WorkspaceCreationFailed(format!("I/O error: {}", err))
    }
}

pub type SelResult<T> = Result<T, SovereignError>;
