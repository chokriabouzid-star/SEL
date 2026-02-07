use serde::Serialize;

/// نتيجة التحقق
#[derive(Debug, Clone, Serialize)]
pub struct ValidationResult {
    pub verdict: Verdict,
    pub rules_applied: usize,
    pub rules_passed: usize,
    pub violations: Vec<Violation>,
}

/// الحكم
#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum Verdict {
    Valid,
    Invalid,
}

/// انتهاك قاعدة
#[derive(Debug, Clone, Serialize)]
pub struct Violation {
    pub rule: String,
    pub location: String,
    pub fact: String,
}

/// خطأ في التحقق
#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),
    
    #[error("Validation failed: {0}")]
    Failed(String),
}
