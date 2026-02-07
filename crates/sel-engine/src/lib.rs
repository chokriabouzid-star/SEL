//! Sovereign Execution Layer - Engine

pub mod engine;
pub mod mission;
pub mod adapter;  // نستخدم adapter الحقيقي

pub use engine::SELEngine;
pub use mission::Mission;
pub use adapter::validate_mission;

/// نتيجة تنفيذ SEL
pub type Result<T> = std::result::Result<T, Error>;

/// أخطاء SEL Engine
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("Validation error:\n{0}")]
    Validation(String),
    
    #[error("Execution error: {0}")]
    Execution(String),
    
    #[error("Fact logging error: {0}")]
    FactLogging(String),
}

/// واجهة برمجية بسيطة للاستخدام الخارجي
pub struct SEL {
    engine: SELEngine,
}

impl SEL {
    /// إنشاء مثيل جديد لـ SEL
    pub fn new(mission_id: &str) -> Result<Self> {
        Ok(SEL {
            engine: SELEngine::new(mission_id)?,
        })
    }
    
    /// تنفيذ مهمة (يدخل مباشرة بعد التحقق)
    pub fn execute(&mut self, mission: Mission) -> Result<()> {
        self.engine.execute_mission(mission)
    }
    
    /// الحصول على مسار ملف الحقائق
    pub fn facts_path(&self) -> &std::path::Path {
        self.engine.facts_path()
    }
}
