//! SEL Engine core components

pub mod facts_logger;
pub mod mission_executor;
pub mod logical_clock;
pub mod types;
pub mod executor;
pub mod workspace;  // ← NEW

// Re-exports
pub use facts_logger::{FactsLogger, FactsLoggerError};
pub use mission_executor::{MissionExecutor as OldMissionExecutor, MissionExecutorError};
pub use logical_clock::LogicalClock;
pub use types::*;
pub use workspace::Workspace;  // ← NEW
pub use executor::MissionExecutor;  // ← NEW (replaces old)
