//! # Resource Kind - SEL Core 1.0
//! أنواع الموارد السيادية - كل نوع له دلالة مستقلة

use std::fmt;

/// نوع المورد الذي تم استنفاذه
/// SEL Core 1.0: دلالة دقيقة لا تقبل الالتباس
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResourceKind {
    /// عدد الأوامر في المهمة
    Actions,
    /// عدد الـ logical ticks المستهلكة
    Ticks,
    /// حجم مخرجات stdout (بالبايت)
    Stdout,
    /// حجم مخرجات stderr (بالبايت)
    Stderr,
    /// عدد الـ Facts المسجلة
    Facts,
}

impl ResourceKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Actions => "actions",
            Self::Ticks => "ticks",
            Self::Stdout => "stdout",
            Self::Stderr => "stderr",
            Self::Facts => "facts",
        }
    }
}

impl fmt::Display for ResourceKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
