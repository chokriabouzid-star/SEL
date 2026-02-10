//! Sovereign Execution Layer - Core Engine
//!
//! This crate provides the deterministic execution engine for SEL.

pub mod engine;

// Re-exports from engine module
pub use engine::{
    FactsLogger,
    LogicalClock,
    MissionExecutor,
    ExecutionReport,
    ActionResult,
    ExecutorError,
    WorkspaceError,
    CapabilityViolation,
};
