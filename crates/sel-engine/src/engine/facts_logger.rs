//! # Facts Logger - Tamper-Evident Fact Chain
//! SEL Core 1.0 - DETERMINISTIC: Same facts = Same final hash
//!
//! 🔴 كل Executor جديد يجب أن يبدأ من Genesis Hash
//! ✅ create_new(true) يضمن عدم reuse لنفس الملف
//!
//! ## Durability
//! `FactsLogger::new()` defaults to `durable = true`:
//! after every fact it calls `flush()` (BufWriter → OS page cache)
//! then `sync_all()` (OS page cache → physical disk).
//! This matches the project's "tamper-evident audit trail" claim.
//! Use `with_durability(path, false)` only when you explicitly accept
//! the risk of losing the last fact(s) on a crash/power loss.

use sel_common::{HashChain, SovereignError};
use serde_json::Value;
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::Path;

/// Tamper-evident facts logger.
/// DETERMINISTIC: Same sequence of logged facts = Same final hash.
pub struct FactsLogger {
    writer: BufWriter<File>,
    hash_chain: HashChain,
    path: std::path::PathBuf,
    /// When `true`, every `log_fact` call fsyncs to physical disk.
    /// When `false`, only flushes to the OS page cache (faster, less safe).
    durable: bool,
}

impl FactsLogger {
    /// Create a new facts logger at `path` with **durability enabled**.
    ///
    /// Every fact is flushed to the OS page cache *and* fsynced to physical
    /// disk before `log_fact` returns.  This is the only mode that justifies
    /// the "tamper-evident audit trail" claim in the project documentation.
    ///
    /// 🔴 Uses `create_new(true)` — fails if the file already exists,
    /// guaranteeing every execution starts from the genesis hash.
    pub fn new(path: impl AsRef<Path>) -> Result<Self, SovereignError> {
        Self::with_durability(path, true)
    }

    /// Create a new facts logger with explicit durability control.
    ///
    /// `durable = true`  → flush + fsync after every fact (safe, default).
    /// `durable = false` → flush to OS page cache only (faster, but a crash
    /// before the OS writes back can silently lose the last fact(s)).
    /// Do **not** use `false` for anything claiming to be an audit trail.
    ///
    /// 🔴 Uses `create_new(true)` — fails if the file already exists.
    pub fn with_durability(
        path: impl AsRef<Path>,
        durable: bool,
    ) -> Result<Self, SovereignError> {
        let path = path.as_ref().to_path_buf();

        // تحقق من وجود المجلد الأصلي
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                return Err(SovereignError::WorkspaceCreationFailed(format!(
                    "Parent directory does not exist: {}",
                    parent.display()
                )));
            }
        }

        // 🔴 create_new(true): فشل إذا كان الملف موجوداً
        // هذا يضمن أن كل تنفيذ يبدأ من genesis hash
        let file = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&path)
            .map_err(|e| {
                SovereignError::WorkspaceCreationFailed(format!(
                    "Failed to create facts file at {}: {}",
                    path.display(),
                    e
                ))
            })?;

        Ok(Self {
            writer: BufWriter::new(file),
            hash_chain: HashChain::new(),
            path,
            durable,
        })
    }

    /// Log a fact.
    /// DETERMINISTIC: Same fact = Same contribution to hash chain.
    ///
    /// When `durable = true` (the default): flushes the `BufWriter` to the
    /// OS page cache, then calls `sync_all()` to commit to physical disk
    /// before returning.  A crash after this call will not lose this fact.
    pub fn log_fact(&mut self, fact: Value) -> Result<String, SovereignError> {
        // Serialize to JSON (deterministic)
        let fact_json = serde_json::to_string(&fact).map_err(|e| {
            SovereignError::InternalError(format!("Failed to serialize fact: {}", e))
        })?;

        // Write to BufWriter
        writeln!(self.writer, "{}", fact_json)
            .map_err(|e| SovereignError::InternalError(format!("Failed to write fact: {}", e)))?;

        // Flush BufWriter → OS page cache
        self.writer
            .flush()
            .map_err(|e| SovereignError::InternalError(format!("Failed to flush: {}", e)))?;

        // fsync OS page cache → physical disk (durability guarantee)
        if self.durable {
            self.writer
                .get_ref()
                .sync_all()
                .map_err(|e| SovereignError::InternalError(format!("Failed to fsync: {}", e)))?;
        }

        // Add to hash chain (deterministic)
        Ok(self.hash_chain.add_fact(&fact))
    }

    /// Finalize and return the final hash.
    pub fn finalize(&self) -> String {
        self.hash_chain.finalize()
    }

    /// Return the path to the facts file.
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

        let _logger1 = FactsLogger::new(&path).unwrap();
        // Second creation must fail (file already exists)
        let _logger2 = FactsLogger::new(&path).unwrap();
    }

    #[test]
    fn test_durable_by_default() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("facts.jsonl");
        let logger = FactsLogger::new(&path).unwrap();
        assert!(
            logger.durable,
            "FactsLogger::new() must default to durable = true \
             to honour the tamper-evident audit-trail claim"
        );
    }

    #[test]
    fn test_fast_mode_writes_correct_data() {
        // durable = false must still write and hash correctly —
        // it only skips the fsync, not the write itself.
        let dir = tempdir().unwrap();
        let path = dir.path().join("facts.jsonl");
        let mut logger = FactsLogger::with_durability(&path, false).unwrap();
        assert!(!logger.durable);

        let fact = serde_json::json!({"type": "test", "value": 1});
        logger.log_fact(fact).unwrap();

        let contents = std::fs::read_to_string(&path).unwrap();
        assert!(
            contents.contains("\"value\":1"),
            "fast-mode logger must still write data to disk"
        );
    }
}
