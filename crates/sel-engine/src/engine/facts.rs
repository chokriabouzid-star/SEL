//! نظام تسجيل الحقائق - JSONL خام

use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use chrono::Utc;
use serde_json::Value;
use crate::Result;

/// مسجل الحقائق
pub struct FactLogger {
    _mission_id: String,  // نستخدم _ للإشارة أنه غير مستخدم حالياً
    file: File,
    file_path: PathBuf,
}

impl FactLogger {
    /// إنشاء مسجل جديد
    pub fn new(mission_id: &str) -> Result<Self> {
        let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
        let filename = format!("facts_{}_{}.jsonl", mission_id, timestamp);
        let file_path = PathBuf::from(&filename);
        
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(&file_path)?;
        
        Ok(FactLogger {
            _mission_id: mission_id.to_string(),
            file,
            file_path,
        })
    }
    
    /// تسجيل حقيقة واحدة
    pub fn log(&mut self, fact_type: &str, mission_id: &str, data: Value) -> Result<()> {
        let fact = serde_json::json!({
            "timestamp": Utc::now().to_rfc3339(),
            "mission_id": mission_id,
            "type": fact_type,
            "data": data,
        });
        
        writeln!(&mut self.file, "{}", fact.to_string())?;
        self.file.flush()?;
        
        Ok(())
    }
    
    /// الحصول على مسار ملف الحقائق
    pub fn path(&self) -> &Path {
        &self.file_path
    }
}
