//! SEL Engine - المحرك الرئيسي للتنفيذ الحتمي

use std::path::{Path, PathBuf};
use crate::mission::Mission;
use crate::Result;

mod executor;
mod facts;
mod sandbox;

pub use executor::CommandExecutor;
pub use facts::FactLogger;

/// SEL Engine - محرك التنفيذ الحتمي
pub struct SELEngine {
    mission_id: String,
    workspace: PathBuf,
    fact_logger: FactLogger,
    executor: CommandExecutor,
}

impl SELEngine {
    /// إنشاء محرك جديد مع عزل كامل
    pub fn new(mission_id: &str) -> Result<Self> {
        // إنشاء مساحة عمل معزولة
        let workspace = sandbox::create_isolated_workspace(mission_id)?;
        
        // إنشاء مسجل الحقائق
        let fact_logger = FactLogger::new(mission_id)?;
        
        // إنشاء منفذ الأوامر مع mission_id
        let executor = CommandExecutor::new(workspace.clone(), mission_id);
        
        Ok(SELEngine {
            mission_id: mission_id.to_string(),
            workspace,
            fact_logger,
            executor,
        })
    }
    
    /// تنفيذ مهمة كاملة (حتمي بحت)
    pub fn execute_mission(&mut self, mission: Mission) -> Result<()> {
        // 1. تسجيل بدء المهمة
        self.log_fact("mission_started", serde_json::json!({
            "mission_id": &mission.id,
            "version": mission.version,
            "actions_count": mission.execution.actions.len(),
        }))?;
        
        // 2. تنفيذ كل أمر
        for action in mission.execution.actions {
            self.execute_action(action)?;
        }
        
        // 3. تسجيل انتهاء المهمة
        self.log_fact("mission_completed", serde_json::json!({
            "mission_id": &mission.id,
        }))?;
        
        Ok(())
    }
    
    /// تنفيذ أمر واحد
    fn execute_action(&mut self, action: crate::mission::Action) -> Result<()> {
        // تسجيل بدء الأمر
        self.log_fact("action_started", serde_json::json!({
            "action_id": action.id,
            "type": action.action_type,
            "command": action.command,
            "working_directory": action.working_directory,
        }))?;
        
        // التنفيذ
        let result = self.executor.execute(&action)?;
        
        // تسجيل النتائج
        self.log_fact("action_completed", serde_json::json!({
            "action_id": action.id,
            "exit_code": result.exit_code,
            "stdout_size_bytes": result.stdout.len(),
            "stderr_size_bytes": result.stderr.len(),
            "stdout_preview": String::from_utf8_lossy(&result.stdout[..std::cmp::min(result.stdout.len(), 100)]).to_string(),
            "stderr_preview": String::from_utf8_lossy(&result.stderr[..std::cmp::min(result.stderr.len(), 100)]).to_string(),
        }))?;
        
        Ok(())
    }
    
    /// تسجيل حقيقة (داخلي)
    fn log_fact(&mut self, fact_type: &str, data: serde_json::Value) -> Result<()> {
        self.fact_logger.log(fact_type, &self.mission_id, data)
    }
    
    /// الحصول على مسار ملف الحقائق
    pub fn facts_path(&self) -> &Path {
        self.fact_logger.path()
    }
    
    /// الحصول على مسار مساحة العمل
    pub fn workspace_path(&self) -> &Path {
        &self.workspace
    }
}
