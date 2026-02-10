//! Workspace Isolation - UUID-based Sovereign Execution Environment

use std::fs;
use std::path::{Path, PathBuf};
use uuid::Uuid;
use sel_validator::WorkspaceMode;
use crate::engine::types::WorkspaceError;

/// Isolated workspace for mission execution
#[derive(Debug)]
pub struct Workspace {
    uuid: Uuid,
    path: PathBuf,
    mode: WorkspaceMode,
}

impl Workspace {
    pub fn new(mode: WorkspaceMode) -> Result<Self, WorkspaceError> {
        let uuid = Uuid::new_v4();
        let path = PathBuf::from(format!("/tmp/sel-workspace-{}", uuid));
        
        fs::create_dir_all(&path)
            .map_err(WorkspaceError::CreationFailed)?;
        
        // Set permissions for ReadOnly
        if matches!(mode, WorkspaceMode::ReadOnly) {
            let mut perms = fs::metadata(&path)
                .map_err(WorkspaceError::PermissionsFailed)?
                .permissions();
            
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                perms.set_mode(0o555);
            }
            
            fs::set_permissions(&path, perms)
                .map_err(WorkspaceError::PermissionsFailed)?;
        }
        
        Ok(Workspace { uuid, path, mode })
    }
    
    pub fn path(&self) -> &Path {
        &self.path
    }
    
    pub fn uuid(&self) -> &Uuid {
        &self.uuid
    }
    
    pub fn mode(&self) -> WorkspaceMode {
        self.mode
    }
    
    pub fn cleanup(&self) -> Result<(), WorkspaceError> {
        if self.path.exists() {
            if matches!(self.mode, WorkspaceMode::ReadOnly) {
                let mut perms = fs::metadata(&self.path)
                    .map_err(WorkspaceError::CleanupFailed)?
                    .permissions();
                
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    perms.set_mode(0o755);
                }
                
                fs::set_permissions(&self.path, perms)
                    .map_err(WorkspaceError::CleanupFailed)?;
            }
            
            fs::remove_dir_all(&self.path)
                .map_err(WorkspaceError::CleanupFailed)?;
        }
        
        Ok(())
    }
}

impl Drop for Workspace {
    fn drop(&mut self) {
        let _ = self.cleanup();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workspace_creation_readonly() {
        let workspace = Workspace::new(WorkspaceMode::ReadOnly).unwrap();
        assert!(workspace.path().exists());
        drop(workspace);
    }

    #[test]
    fn test_workspace_isolation() {
        let ws1 = Workspace::new(WorkspaceMode::ReadOnly).unwrap();
        let ws2 = Workspace::new(WorkspaceMode::ReadOnly).unwrap();
        
        assert_ne!(ws1.uuid(), ws2.uuid());
        assert_ne!(ws1.path(), ws2.path());
        assert!(ws1.path().exists());
        assert!(ws2.path().exists());
    }

    #[test]
    fn test_readonly_workspace() {
        let workspace = Workspace::new(WorkspaceMode::ReadOnly).unwrap();
        
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let metadata = fs::metadata(workspace.path()).unwrap();
            let mode = metadata.permissions().mode();
            assert_eq!(mode & 0o777, 0o555);
        }
    }

    #[test]
    fn test_automatic_cleanup() {
        let path = {
            let workspace = Workspace::new(WorkspaceMode::ReadOnly).unwrap();
            workspace.path().to_path_buf()
        };
        
        std::thread::sleep(std::time::Duration::from_millis(100));
        assert!(!path.exists());
    }
}
