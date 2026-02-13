//! # Sovereign Workspace Isolation
//! SEL Core 1.0 - DETERMINISTIC UUID (v5, no randomness)

use std::path::{Path, PathBuf};
use std::fs;
use uuid::Uuid;
use sel_validator::WorkspaceMode;
use sel_common::SovereignError;

/// DETERMINISTIC namespace for SEL workspaces
/// This is FIXED and NEVER changes (like a constant)
const SEL_NAMESPACE: Uuid = Uuid::from_bytes([
    0x6b, 0xa7, 0xb8, 0x10, 0x9d, 0xad, 0x11, 0xd1,
    0x80, 0xb4, 0x00, 0xc0, 0x4f, 0xd4, 0x30, 0xc8,
]);

/// Isolated workspace for deterministic execution
/// 🔴 NO RANDOMNESS: UUID is derived deterministically from mission hash
pub struct Workspace {
    pub uuid: Uuid,
    pub path: PathBuf,
    pub mode: WorkspaceMode,
}

impl Workspace {
    /// Create new workspace with DETERMINISTIC UUID
    /// UUID = v5(mission_hash) → always the same for same mission
    pub fn new(mode: WorkspaceMode, mission_hash: &str) -> Result<Self, SovereignError> {
        // 🔴🔴🔴 DETERMINISTIC UUID - NO RANDOMNESS
        let uuid = Uuid::new_v5(&SEL_NAMESPACE, mission_hash.as_bytes());
        
        let base_path = std::env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("."))
            .join("target")
            .join("sel_workspaces")
            .join(uuid.to_string());
        
        fs::create_dir_all(&base_path)
            .map_err(|e| SovereignError::WorkspaceCreationFailed(
                format!("Failed to create workspace at {}: {}", base_path.display(), e)
            ))?;
        
        Ok(Self {
            uuid,
            path: base_path,
            mode,
        })
    }
    
    pub fn uuid(&self) -> Uuid {
        self.uuid
    }
    
    pub fn path(&self) -> &Path {
        &self.path
    }
    
    pub fn cleanup(&mut self) -> Result<(), SovereignError> {
        if self.path.exists() {
            let _ = fs::remove_dir_all(&self.path);
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
    fn test_workspace_deterministic_uuid() {
        let ws1 = Workspace::new(WorkspaceMode::ReadOnly, "test-mission-123").unwrap();
        let ws2 = Workspace::new(WorkspaceMode::ReadOnly, "test-mission-123").unwrap();
        
        // Same mission hash = Same UUID (DETERMINISTIC)
        assert_eq!(ws1.uuid(), ws2.uuid());
        
        let ws3 = Workspace::new(WorkspaceMode::ReadOnly, "different-mission").unwrap();
        // Different mission hash = Different UUID
        assert_ne!(ws1.uuid(), ws3.uuid());
    }
}
