//! Legacy mission executor (from previous days)
//! Keeping for compatibility

use thiserror::Error;

#[derive(Error, Debug)]
pub enum MissionExecutorError {
    #[error("Execution failed: {0}")]
    ExecutionFailed(String),
}

pub struct MissionExecutor;

impl MissionExecutor {
    pub fn new() -> Self {
        Self
    }
}
