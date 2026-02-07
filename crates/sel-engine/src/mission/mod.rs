//! تعريفات هياكل المهام

use serde::{Deserialize, Serialize};

/// مهمة SEL
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mission {
    pub id: String,
    pub version: String,
    #[serde(default = "default_metadata")]  // قيمة افتراضية إذا لم يكن موجوداً
    pub metadata: serde_json::Value,
    pub execution: Execution,
}

/// تنفيذ المهمة
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Execution {
    pub actions: Vec<Action>,
}

/// إجراء واحد
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    pub id: u32,
    #[serde(rename = "type")]
    pub action_type: String,
    pub command: String,
    #[serde(default)]
    pub args: Option<Vec<String>>,
    pub working_directory: String,
    #[serde(default)]
    pub environment: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    pub verification: Option<serde_json::Value>,
}

/// قيمة افتراضية لـ metadata
fn default_metadata() -> serde_json::Value {
    serde_json::json!({
        "source": "unknown",
        "timestamp": chrono::Utc::now().to_rfc3339()
    })
}
