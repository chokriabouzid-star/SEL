//! Legacy mission executor (from previous days)
//! Keeping for compatibility

#[derive(Debug)]
pub enum MissionExecutorError {
    ExecutionFailed(String),
}

impl std::fmt::Display for MissionExecutorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MissionExecutorError::ExecutionFailed(msg) => {
                write!(f, "Execution failed: {}", msg)
            }
        }
    }
}

impl std::error::Error for MissionExecutorError {}

pub struct MissionExecutor;

impl MissionExecutor {
    pub fn new() -> Self {
        Self
    }
}
