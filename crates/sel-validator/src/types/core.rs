use serde::{Deserialize, Serialize};

/// Mission - الهيكل الرئيسي
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mission {
    pub id: String,
    pub version: String,
    
    #[serde(default)]
    pub metadata: serde_json::Value,  // Opaque - لا يُقرأ
    
    pub execution: ExecutionPlan,
}

/// خطة التنفيذ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionPlan {
    pub actions: Vec<Action>,
    
    #[serde(default)]
    pub completion: CompletionConditions,
}

/// إجراء فردي
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    pub id: usize,
    
    #[serde(rename = "type")]
    pub action_type: String,  // يجب أن تكون "command" فقط في v0.1
    
    pub command: String,
    
    #[serde(default = "default_working_directory")]
    pub working_directory: String,
    
    #[serde(default = "default_timeout")]
    pub timeout_seconds: u64,
    
    pub verification: Verification,
}

fn default_working_directory() -> String {
    "/workspace/${mission_id}".to_string()
}

fn default_timeout() -> u64 {
    30
}

/// التحقق الميكانيكي
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Verification {
    #[serde(default = "default_exit_code")]
    pub exit_code: i32,
    
    #[serde(default)]
    pub file_must_exist: Option<String>,
    
    #[serde(default)]
    pub output_must_contain: Option<String>,
}

fn default_exit_code() -> i32 {
    0
}

/// شروط الإكمال
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CompletionConditions {
    #[serde(default = "default_all_must_complete")]
    pub all_actions_must_complete: bool,
    
    #[serde(default)]
    pub required_artifacts: Vec<ArtifactRequirement>,
}

fn default_all_must_complete() -> bool {
    true
}

/// متطلبات القطع الأثرية
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactRequirement {
    pub path: String,
    
    #[serde(default)]
    pub hash: Option<String>,
}
