//! # Facts Logger - Tamper-Evident Fact Chain
//! SEL Core 1.0 - DETERMINISTIC: Same facts = Same final hash
//! 
//! 🔴 كل Executor جديد يجب أن يبدأ من Genesis Hash
//! ✅ create_new(true) يضمن عدم reuse لنفس الملف

use std::fs::{File, OpenOptions};
use std::io::{Write, BufWriter};
use std::path::Path;
use serde_json::Value;
use sel_common::{HashChain, SovereignError};

/// Tamper-evident facts logger
/// DETERMINISTIC: Same sequence of logged facts = Same final hash
pub struct FactsLogger {
    writer: BufWriter<File>,
    hash_chain: HashChain,
    path: std::path::PathBuf,
}

impl FactsLogger {
    /// Create new facts logger at specified path
    /// 🔴 يستخدم create_new(true) - يفشل إذا كان الملف موجوداً
    pub fn new(path: impl AsRef<Path>) -> Result<Self, SovereignError> {
        let path = path.as_ref().to_path_buf();
        
        // تحقق من وجود المجلد الأصلي
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                return Err(SovereignError::WorkspaceCreationFailed(
                    format!("Parent directory does not exist: {}", parent.display())
                ));
            }
        }
        
        // 🔴 create_new(true): فشل إذا كان الملف موجوداً
        // هذا يضمن أن كل تنفيذ يبدأ من genesis hash
        let file = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&path)
            .map_err(|e| SovereignError::WorkspaceCreationFailed(
                format!("Failed to create facts file at {}: {}", path.display(), e)
            ))?;
        
        Ok(Self {
            writer: BufWriter::new(file),
            hash_chain: HashChain::new(),
            path,
        })
    }
    
    /// Log a fact
    /// DETERMINISTIC: Same fact = Same contribution to hash chain
    pub fn log_fact(&mut self, fact: Value) -> Result<String, SovereignError> {
        // Convert to JSON string (deterministic)
        let fact_json = serde_json::to_string(&fact)
            .map_err(|e| SovereignError::InternalError(format!("Failed to serialize fact: {}", e)))?;
        
        // Write to file
        writeln!(self.writer, "{}", fact_json)
            .map_err(|e| SovereignError::InternalError(format!("Failed to write fact: {}", e)))?;
        
        // Flush
        self.writer.flush()
            .map_err(|e| SovereignError::InternalError(format!("Failed to flush: {}", e)))?;
        
        // Add to hash chain (deterministic)
        Ok(self.hash_chain.add_fact(&fact))
    }
    
    /// Finalize and get final hash
    pub fn finalize(&self) -> String {
        self.hash_chain.finalize()
    }
    
    /// Get path to facts file
    pub fn path(&self) -> &std::path::Path {
        &self.path
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    
    #[test]
    fn test_log_fact_deterministic() {
        let dir = tempdir().unwrap();
        let path1 = dir.path().join("facts1.jsonl");
        let path2 = dir.path().join("facts2.jsonl");
        
        let mut logger1 = FactsLogger::new(&path1).unwrap();
        let mut logger2 = FactsLogger::new(&path2).unwrap();
        
        let fact = serde_json::json!({"type": "test", "value": 42});
        
        let hash1 = logger1.log_fact(fact.clone()).unwrap();
        let hash2 = logger2.log_fact(fact).unwrap();
        
        assert_eq!(hash1, hash2);
        assert_eq!(logger1.finalize(), logger2.finalize());
    }
    
    #[test]
    #[should_panic(expected = "WorkspaceCreationFailed")]
    fn test_cannot_reuse_file() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("facts.jsonl");
        
        // First creation succeeds
        let _logger1 = FactsLogger::new(&path).unwrap();
        
        // Second creation should fail (file exists)
        let _logger2 = FactsLogger::new(&path).unwrap();
    }
}
