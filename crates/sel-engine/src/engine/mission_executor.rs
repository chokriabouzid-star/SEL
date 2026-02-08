//! Mission executor with isolated workspace
//!
//! Provides deterministic execution environment with UUID isolation

use uuid::Uuid;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use thiserror::Error;
use serde_json::json;

use crate::engine::FactsLogger;
use sel_core::normalize_command_env;

#[derive(Error, Debug)]
pub enum MissionExecutorError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Workspace error: {0}")]
    Workspace(String),

    #[error("Execution error: {0}")]
    Execution(String),
    
    #[error("Facts logger error: {0}")]
    FactsLogger(#[from] crate::engine::facts_logger::FactsLoggerError),
}

/// Mission executor with isolated workspace
pub struct MissionExecutor {
    mission_id: String,
    workspace: PathBuf,
    facts_logger: FactsLogger,
    workspace_uuid: Uuid,
}

impl MissionExecutor {
    /// Create new mission executor with unique workspace
    pub fn new(mission_id: &str) -> Result<Self, MissionExecutorError> {
        // Generate unique UUID for this execution
        let workspace_uuid = Uuid::new_v4();

        // Create unique workspace path
        let workspace = PathBuf::from(format!("/tmp/sel-workspace-{}", workspace_uuid));

        // Create workspace directory
        fs::create_dir_all(&workspace)
            .map_err(|e| MissionExecutorError::Workspace(format!("Failed to create workspace: {}", e)))?;

        // Create facts log path (in current directory, not workspace)
        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
        let facts_log_name = format!("facts_{}_{}.jsonl", mission_id, timestamp);
        let facts_log_path = PathBuf::from(&facts_log_name);

        // Create facts logger
        let facts_logger = FactsLogger::new(facts_log_path.to_str().unwrap())?;

        println!("🔧 Created isolated workspace: {}", workspace.display());
        println!("   UUID: {}", workspace_uuid);
        println!("📊 Facts log: {}", facts_log_name);

        Ok(MissionExecutor {
            mission_id: mission_id.to_string(),
            workspace,
            facts_logger,
            workspace_uuid,
        })
    }

    /// Get workspace path
    pub fn workspace_path(&self) -> &Path {
        &self.workspace
    }

    /// Get workspace UUID
    pub fn workspace_uuid(&self) -> &Uuid {
        &self.workspace_uuid
    }

    /// Get facts logger (mutable)
    pub fn facts_logger(&mut self) -> &mut FactsLogger {
        &mut self.facts_logger
    }

    /// Execute a command in the isolated workspace with sovereign environment
    pub fn execute_command(&mut self, command: &str, args: &[&str]) -> Result<(), MissionExecutorError> {
        // Log execution start
        let start_fact = json!({
            "type": "command_start",
            "command": command,
            "args": args,
            "workspace": self.workspace.to_str().unwrap(),
            "workspace_uuid": self.workspace_uuid.to_string(),
        });

        self.facts_logger.log_fact(&start_fact)?;

        // Create command
        let mut cmd = Command::new(command);
        cmd.args(args);
        cmd.current_dir(&self.workspace);

        // ✅ ENFORCE sovereign environment normalization
        normalize_command_env(&mut cmd);

        // Execute command
        let output = cmd.output()
            .map_err(|e| MissionExecutorError::Execution(format!("Command execution failed: {}", e)))?;

        // Log execution result
        let result_fact = json!({
            "type": "command_result",
            "command": command,
            "exit_code": output.status.code().unwrap_or(-1),
            "stdout_size": output.stdout.len(),
            "stderr_size": output.stderr.len(),
            "success": output.status.success(),
        });

        self.facts_logger.log_fact(&result_fact)?;

        if !output.status.success() {
            return Err(MissionExecutorError::Execution(format!(
                "Command failed with exit code {:?}",
                output.status.code()
            )));
        }

        Ok(())
    }

    /// Cleanup workspace (called automatically on drop, but can be manual)
    pub fn cleanup(&self) -> Result<(), MissionExecutorError> {
        if self.workspace.exists() {
            fs::remove_dir_all(&self.workspace)
                .map_err(|e| MissionExecutorError::Workspace(format!("Failed to cleanup workspace: {}", e)))?;
            println!("🧹 Cleaned up workspace: {}", self.workspace.display());
        }
        Ok(())
    }

    /// Get final facts hash
    pub fn final_facts_hash(&self) -> String {
        self.facts_logger.final_hash()
    }

    /// Get facts count
    pub fn facts_count(&self) -> usize {
        self.facts_logger.fact_count()
    }
}

impl Drop for MissionExecutor {
    fn drop(&mut self) {
        // Attempt cleanup, but don't panic if it fails
        if let Err(e) = self.cleanup() {
            eprintln!("⚠️ Warning: Failed to cleanup workspace {}: {}",
                     self.workspace.display(), e);
            // Don't panic in drop - just log warning
            // Workspace will be cleaned up manually or by system temp cleanup
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mission_executor_creation() {
        let executor = MissionExecutor::new("test-mission");
        assert!(executor.is_ok());

        let executor = executor.unwrap();
        assert!(executor.workspace_path().exists());
        assert!(executor.workspace_path().is_dir());

        // Workspace should have UUID format
        let workspace_str = executor.workspace_path().to_str().unwrap();
        assert!(workspace_str.contains("sel-workspace-"));
        assert!(workspace_str.contains(executor.workspace_uuid().to_string().as_str()));
    }

    #[test]
    fn test_workspace_isolation() {
        // Create two executors - should have different workspaces
        let exec1 = MissionExecutor::new("mission-1").unwrap();
        let exec2 = MissionExecutor::new("mission-2").unwrap();

        let path1 = exec1.workspace_path();
        let path2 = exec2.workspace_path();

        assert_ne!(path1, path2, "Workspaces should be different");
        assert_ne!(exec1.workspace_uuid(), exec2.workspace_uuid());

        // Both workspaces should exist
        assert!(path1.exists());
        assert!(path2.exists());
    }
}
