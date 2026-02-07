//! منفذ الأوامر - تنفيذ حرفي بدون ذكاء

use std::process::{Command, Stdio};
use crate::mission::Action;
use crate::Result;

/// نتيجة تنفيذ الأمر
#[derive(Debug)]
pub struct CommandResult {
    pub exit_code: Option<i32>,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
}

/// منفذ الأوامر الحتمي
pub struct CommandExecutor {
    workspace: std::path::PathBuf,
    mission_id: String,
}

impl CommandExecutor {
    /// إنشاء منفذ جديد
    pub fn new(workspace: std::path::PathBuf, mission_id: &str) -> Self {
        CommandExecutor { 
            workspace,
            mission_id: mission_id.to_string(),
        }
    }
    
    /// تنفيذ أمر واحد (ميكانيكي بحت)
    pub fn execute(&self, action: &Action) -> Result<CommandResult> {
        // استبدال المتغيرات في مسار العمل
        let resolved_path = self.resolve_working_directory(&action.working_directory);
        
        // إنشاء المجلد إذا لم يكن موجوداً
        self.ensure_directory_exists(&resolved_path)?;
        
        // التحقق من المسار
        self.validate_working_directory(&resolved_path)?;
        
        // بناء الأمر
        let mut command = Command::new(&action.command);
        
        // إضافة الوسيطات
        if let Some(args) = &action.args {
            // استبدال المتغيرات في الوسيطات أيضاً
            let resolved_args: Vec<String> = args.iter()
                .map(|arg| self.resolve_variables(arg))
                .collect();
            command.args(resolved_args);
        }
        
        // تعيين البيئة
        if let Some(env) = &action.environment {
            for (key, value) in env {
                command.env(key, value);
            }
        }
        
        // تعيين مسار العمل (المسار الكامل داخل workspace)
        let working_dir = self.workspace.join(&resolved_path);
        command.current_dir(working_dir);
        
        // إعداد الإخراج
        command.stdout(Stdio::piped());
        command.stderr(Stdio::piped());
        
        // التنفيذ
        let output = command.output()?;
        
        Ok(CommandResult {
            exit_code: output.status.code(),
            stdout: output.stdout,
            stderr: output.stderr,
        })
    }
    
    /// استبدال المتغيرات في النص
    fn resolve_variables(&self, text: &str) -> String {
        text.replace("${mission_id}", &self.mission_id)
            .replace("${workspace}", self.workspace.to_str().unwrap_or(""))
    }
    
    /// استبدال المتغيرات في مسار العمل
    fn resolve_working_directory(&self, path: &str) -> String {
        let resolved = self.resolve_variables(path);
        
        // إذا بدأ بـ /workspace/، أزل /workspace/ الأول
        if resolved.starts_with("/workspace/") {
            resolved.trim_start_matches("/workspace/").to_string()
        } else if resolved.starts_with("workspace/") {
            resolved.trim_start_matches("workspace/").to_string()
        } else {
            resolved
        }
    }
    
    /// إنشاء المجلد إذا لم يكن موجوداً
    fn ensure_directory_exists(&self, path: &str) -> Result<()> {
        let full_path = self.workspace.join(path);
        
        if !full_path.exists() {
            std::fs::create_dir_all(&full_path)?;
        }
        
        Ok(())
    }
    
    /// التحقق من مسار العمل (داخل workspace فقط)
    fn validate_working_directory(&self, resolved_path: &str) -> Result<()> {
        let full_path = self.workspace.join(resolved_path);
        
        // التأكد أن المسار داخل workspace
        if !full_path.starts_with(&self.workspace) {
            return Err(crate::Error::Execution(
                format!("Working directory outside workspace: {} (full: {})", 
                       resolved_path, full_path.display())
            ));
        }
        
        // التأكد أن المسار لا يحتوي على `..`
        if resolved_path.contains("..") {
            return Err(crate::Error::Execution(
                format!("Working directory contains '..': {}", resolved_path)
            ));
        }
        
        Ok(())
    }
}
