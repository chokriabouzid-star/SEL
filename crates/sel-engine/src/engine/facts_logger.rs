//! Facts Logger for recording execution events

use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FactsLoggerError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    SerializationError(String),
}

pub struct FactsLogger {
    file_path: PathBuf,
}

impl FactsLogger {
    pub fn new(path: PathBuf) -> Result<Self, FactsLoggerError> {
        Ok(Self { file_path: path })
    }
}
