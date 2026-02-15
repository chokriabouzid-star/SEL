//! # SEL Engine
//! Sovereign Execution Layer - Core Execution Engine
//! SEL Core 1.0 Compliant

pub mod engine;

pub use engine::{
    ActionResult,
    ExecutionReport,
    ExecutorError, // 👈 Added for compatibility
    FactsLogger,
    LogicalClock,
    MissionExecutor,
    ResourceLimits,
    Workspace,
    WorkspaceMode,
};
pub use sel_common::SovereignError;
