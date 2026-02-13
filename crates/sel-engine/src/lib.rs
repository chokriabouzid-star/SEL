//! # SEL Engine
//! Sovereign Execution Layer - Core Execution Engine
//! SEL Core 1.0 Compliant

pub mod engine;

pub use engine::{
    MissionExecutor,
    Workspace,
    LogicalClock,
    FactsLogger,
    ExecutionReport,
    ActionResult,
    WorkspaceMode,
    ResourceLimits,
    ExecutorError,      // 👈 Added for compatibility
};
pub use sel_common::SovereignError;
