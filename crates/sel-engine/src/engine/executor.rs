//! Execution Engine with Sovereign DNA

use std::process::{Command, Output};
use std::path::Path;
use sel_core::normalize_command_env;
use crate::Result;

pub struct Executor {
    workspace_path: String,
}

impl Executor {
    pub fn new(workspace_path: &str) -> Self {
        Self {
            workspace_path: workspace_path.to_string(),
        }
    }
    
    /// Execute command with sovereign environment
    pub fn execute_command(
        &self,
        command: &str,
        args: &[String],
        working_dir: Option<&str>,
    ) -> Result<Output> {
        let mut cmd = Command::new(command);
        
        // 1. Add arguments
        if !args.is_empty() {
            cmd.args(args);
        }
        
        // 2. Set working directory
        let work_dir = working_dir
            .unwrap_or(&self.workspace_path);
        cmd.current_dir(work_dir);
        
        // 3. ✅ ENFORCE sovereign environment
        normalize_command_env(&mut cmd);
        
        // 4. Execute
        let output = cmd.output()
            .map_err(|e| format!("Command execution failed: {}", e))?;
        
        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_executor_creation() {
        let executor = Executor::new("/tmp/test");
        assert_eq!(executor.workspace_path, "/tmp/test");
    }
}
