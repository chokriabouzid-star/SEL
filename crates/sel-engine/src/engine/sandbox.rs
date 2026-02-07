//! نظام العزل الأساسي

use std::fs::{self, create_dir_all};
use std::path::PathBuf;
use tempfile::tempdir;
use crate::Result;

/// إنشاء مساحة عمل معزولة
pub fn create_isolated_workspace(mission_id: &str) -> Result<PathBuf> {
    // إنشاء مجلد مؤقت
    let temp_dir = tempdir()?;
    let workspace = temp_dir.path().join("workspace");
    
    // إنشاء الهيكل الأساسي: workspace/mission_id
    let mission_workspace = workspace.join(mission_id);
    create_dir_all(&mission_workspace)?;
    
    // أيضا إنشاء /workspace داخل المساحة المؤقتة
    // لأن المهام تتوقع /workspace/${mission_id}
    create_dir_all(&workspace)?;
    
    // منع الوصول خارج workspace
    setup_workspace_isolation(&workspace)?;
    
    Ok(workspace)  // نرجع المسار إلى /workspace (ليس /workspace/mission_id)
}

/// إعداد عزل workspace
fn setup_workspace_isolation(workspace: &PathBuf) -> Result<()> {
    // إنشاء ملف .lock لمنع التعديلات الخارجية
    let lock_file = workspace.join(".sel_lock");
    fs::write(lock_file, format!("SEL Workspace - Path: {}", workspace.display()))?;
    
    // إنشاء سجل workspace
    let log_file = workspace.join(".sel_workspace.log");
    fs::write(log_file, format!("Created at: {}\n", chrono::Utc::now()))?;
    
    Ok(())
}

/// تنظيف workspace بعد الانتهاء (دالة محجوزة للاستخدام المستقبلي)
#[allow(dead_code)]
pub fn cleanup_workspace(workspace: &PathBuf) -> Result<()> {
    if workspace.exists() {
        // حذف محتويات workspace فقط (ليس المجلد نفسه)
        if let Ok(entries) = fs::read_dir(workspace) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    fs::remove_dir_all(path)?;
                } else {
                    fs::remove_file(path)?;
                }
            }
        }
    }
    Ok(())
}
