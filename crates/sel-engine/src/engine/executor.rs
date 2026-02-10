//! Mission Executor - Type-Safe Sovereign Execution

use uuid::Uuid;
use std::path::PathBuf;
use sel_validator::{ValidatedMission, ExecutionCapabilities, WorkspaceMode};
use crate::engine::{
    types::*,
    workspace::Workspace,
    FactsLogger,
    LogicalClock,
};

pub struct MissionExecutor {
    workspace: Workspace,
    facts_logger: FactsLogger,
    logical_clock: LogicalClock,
}

impl MissionExecutor {
    pub fn new(mode: WorkspaceMode) -> Result<Self, ExecutorError> {
        let workspace = Workspace::new(mode)?;
        let facts_path = workspace.path().join("facts.jsonl");
        let facts_logger = FactsLogger::new(facts_path)?;
        let logical_clock = LogicalClock::new();
        
        Ok(MissionExecutor {
            workspace,
            facts_logger,
            logical_clock,
        })
    }
    
    pub fn workspace_uuid(&self) -> &Uuid {
        self.workspace.uuid()
    }
    
    pub fn workspace_path(&self) -> &std::path::Path {
        self.workspace.path()
    }
    
    pub fn facts_path(&self) -> PathBuf {
        self.workspace.path().join("facts.jsonl")
    }
    
    pub fn execute(
        &mut self,
        _validated: ValidatedMission
    ) -> Result<ExecutionReport, ExecutorError> {
        todo!("Step 2-3: Full execution pipeline")
    }
    
    fn execute_action(
        &mut self,
        _action_index: usize,
        _action: &serde_json::Value,
        _capabilities: &ExecutionCapabilities,
    ) -> Result<ActionResult, ExecutorError> {
        todo!("Step 3: Action execution")
    }
    
    fn enforce_capabilities(
        &self,
        _action: &serde_json::Value,
        _capabilities: &ExecutionCapabilities,
    ) -> Result<(), CapabilityViolation> {
        todo!("Step 3: Capability enforcement")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_executor_creation() {
        let executor = MissionExecutor::new(WorkspaceMode::ReadOnly);
        assert!(executor.is_ok());
    }

    #[test]
    fn test_executor_workspace_isolation() {
        let exec1 = MissionExecutor::new(WorkspaceMode::ReadOnly).unwrap();
        let exec2 = MissionExecutor::new(WorkspaceMode::ReadOnly).unwrap();
        
        assert_ne!(exec1.workspace_uuid(), exec2.workspace_uuid());
        assert_ne!(exec1.workspace_path(), exec2.workspace_path());
    }

    #[test]
    fn test_executor_facts_logger() {
        let executor = MissionExecutor::new(WorkspaceMode::ReadOnly).unwrap();
        
        let facts_path = executor.facts_path();
        assert!(facts_path.starts_with(executor.workspace_path()));
        assert!(facts_path.ends_with("facts.jsonl"));
    }

    #[test]
    fn test_executor_readonly_mode() {
        let executor = MissionExecutor::new(WorkspaceMode::ReadOnly).unwrap();
        assert!(executor.workspace_path().exists());
        
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let metadata = std::fs::metadata(executor.workspace_path()).unwrap();
            let mode = metadata.permissions().mode();
            assert_eq!(mode & 0o777, 0o555);
        }
    }

    #[test]
    fn test_executor_cleanup() {
        let path = {
            let executor = MissionExecutor::new(WorkspaceMode::ReadOnly).unwrap();
            executor.workspace_path().to_path_buf()
        };
        
        std::thread::sleep(std::time::Duration::from_millis(100));
        assert!(!path.exists());
    }
}
