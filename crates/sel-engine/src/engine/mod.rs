//! SEL Engine - Sovereign Execution Core
//! SEL Core 1.0 Compliant

pub mod executor;
pub mod facts_logger;
pub mod logical_clock;
pub mod resource_limits;
pub mod types;
pub mod workspace;

pub use executor::MissionExecutor;
pub use facts_logger::FactsLogger;
pub use logical_clock::LogicalClock;
pub use resource_limits::ResourceLimits;
pub use types::{ActionResult, ExecutionReport, ExecutorError};
pub use workspace::Workspace;

// Re-export WorkspaceMode from validator for convenience
pub use sel_validator::WorkspaceMode;
