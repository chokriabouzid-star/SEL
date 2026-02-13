//! SEL Engine - Sovereign Execution Core
//! SEL Core 1.0 Compliant

pub mod types;
pub mod workspace;
pub mod logical_clock;
pub mod facts_logger;
pub mod executor;
pub mod resource_limits;

pub use types::{ExecutionReport, ActionResult, ExecutorError};
pub use workspace::Workspace;
pub use logical_clock::LogicalClock;
pub use facts_logger::FactsLogger;
pub use executor::MissionExecutor;
pub use resource_limits::ResourceLimits;

// Re-export WorkspaceMode from validator for convenience
pub use sel_validator::WorkspaceMode;
