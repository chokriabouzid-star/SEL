//! Facts logging with hash chain integration
//!
//! Provides tamper-proof logging for SEL execution events
//!
//! Features:
//! 1. Each fact gets a unique event_hash
//! 2. Hash chain ensures tamper detection
//! 3. Deterministic logging with fsync guarantee

use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use serde_json::{Value, json};
use thiserror::Error;

use crate::canonical_adapter;

#[derive(Error, Debug)]
pub enum FactsLoggerError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Hash chain error: {0}")]
    HashChain(String),

    #[error("Failed to flush/sync facts: {0}")]
    FlushSync(String),
}

/// Facts logger with hash chain for tamper detection
pub struct FactsLogger {
    file: File,
    hash_chain: sel_core::HashChain,
    log_path: PathBuf,
}

impl FactsLogger {
    /// Create new facts logger
    pub fn new(log_path: &str) -> Result<Self, FactsLoggerError> {
        // Open or create log file
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_path)?;

        let hash_chain = canonical_adapter::create_hash_chain();

        Ok(FactsLogger {
            file,
            hash_chain,
            log_path: PathBuf::from(log_path),
        })
    }

    /// Log a fact with automatic hash chain integration
    /// 🔴 CRITICAL: Uses flush() + sync_all() for durability guarantee
    pub fn log_fact(&mut self, fact: &Value) -> Result<String, FactsLoggerError> {
        // Clone fact to add hash
        let mut fact_with_hash = fact.clone();

        // Add hash chain entry
        let event_hash = self.hash_chain.append(fact);
        fact_with_hash["event_hash"] = json!(event_hash.clone());

        // Add timestamp if not present
        if !fact_with_hash.as_object().unwrap().contains_key("timestamp") {
            let timestamp = chrono::Utc::now().to_rfc3339();
            fact_with_hash["timestamp"] = json!(timestamp);
        }

        // Write to log file (one JSON per line)
        let json_line = serde_json::to_string(&fact_with_hash)?;

        writeln!(self.file, "{}", json_line)?;

        // 🔴 CRITICAL 1: Flush buffer to OS
        self.file.flush()
            .map_err(|e| FactsLoggerError::FlushSync(format!("flush failed: {}", e)))?;

        // 🔴 CRITICAL 2: Sync to physical disk (fsync)
        self.file.sync_all()
            .map_err(|e| FactsLoggerError::FlushSync(format!("sync_all failed: {}", e)))?;

        Ok(event_hash)
    }

    /// Log a fact with custom type
    pub fn log_typed_fact(&mut self, fact_type: &str, data: Value) -> Result<String, FactsLoggerError> {
        let fact = json!({
            "type": fact_type,
            "data": data,
        });

        self.log_fact(&fact)
    }

    /// Get final hash chain state
    pub fn final_hash(&self) -> String {
        self.hash_chain.finalize()
    }

    /// Get number of facts logged
    pub fn fact_count(&self) -> usize {
        self.hash_chain.len()
    }

    /// Get log path
    pub fn log_path(&self) -> &PathBuf {
        &self.log_path
    }

    /// Verify log integrity
    pub fn verify_integrity(&self) -> Result<bool, FactsLoggerError> {
        // Read all lines from log file
        let content = std::fs::read_to_string(&self.log_path)?;
        let lines: Vec<&str> = content.lines().collect();

        // For now, just check that we have same number of lines as facts
        // In a real implementation, we would re-compute the hash chain
        Ok(lines.len() == self.fact_count())
    }

    /// Close the logger and get final state
    pub fn close(mut self) -> Result<(String, usize), FactsLoggerError> {
        let final_hash = self.final_hash();
        let count = self.fact_count();

        // Final flush/sync before closing
        self.file.flush()
            .map_err(|e| FactsLoggerError::FlushSync(format!("final flush failed: {}", e)))?;

        self.file.sync_all()
            .map_err(|e| FactsLoggerError::FlushSync(format!("final sync_all failed: {}", e)))?;

        // File is automatically closed when it drops
        Ok((final_hash, count))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use tempfile::NamedTempFile;

    #[test]
    fn test_create_facts_logger() {
        let temp_file = NamedTempFile::new().unwrap();
        let logger = FactsLogger::new(temp_file.path().to_str().unwrap());

        assert!(logger.is_ok());
    }

    #[test]
    fn test_log_fact_with_fsync() {
        let temp_file = NamedTempFile::new().unwrap();
        let mut logger = FactsLogger::new(temp_file.path().to_str().unwrap()).unwrap();

        let fact = json!({
            "action": "test_action",
            "result": "success"
        });

        let result = logger.log_fact(&fact);
        assert!(result.is_ok());

        let event_hash = result.unwrap();
        assert!(!event_hash.is_empty());
        assert_eq!(logger.fact_count(), 1);

        // Verify file actually contains the data
        let content = std::fs::read_to_string(temp_file.path()).unwrap();
        assert!(content.contains(&event_hash[0..8]));
    }

    #[test]
    fn test_integrity_after_crash_simulation() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_str().unwrap().to_string();

        {
            let mut logger = FactsLogger::new(&path).unwrap();

            // Log some facts
            for i in 0..3 {
                let fact = json!({"index": i});
                logger.log_fact(&fact).unwrap();
            }

            // Simulate crash - logger drops without explicit close
            // But flush/sync_all should have already persisted data
        }

        // After "crash", verify file exists and has content
        let content = std::fs::read_to_string(&path).unwrap();
        let lines: Vec<&str> = content.lines().collect();
        assert_eq!(lines.len(), 3, "All facts should survive crash");

        // Each line should be valid JSON
        for line in lines {
            let parsed: Value = serde_json::from_str(line).unwrap();
            assert!(parsed.get("event_hash").is_some());
        }
    }
}
