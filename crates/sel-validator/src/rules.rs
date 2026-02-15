//! # Security Rules Engine
//! SEL Core 1.0 - Path Traversal & Security Patterns
//!
//! ملاحظة: في Core 1.0، الأوامر المحظورة يتم منعها في Validator أولاً.
//! هذا الملف يركز على الأمان السياقي (path traversal, patterns).

use crate::types::ValidatedAction;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SecurityViolation {
    PathTraversal(String),
    DangerousPattern(String),
}

/// Validate security rules for actions
/// Returns Some(violation) if rule is broken, None if safe
///
/// Core 1.0:
/// - Path traversal detection
/// - Dangerous patterns in commands
/// - Command validation happens in Validator
pub fn validate_security_rules(actions: &[ValidatedAction]) -> Option<SecurityViolation> {
    for action in actions {
        // Check for path traversal in args
        for arg in &action.args {
            if arg.contains("..") || arg.starts_with('/') || arg.starts_with("~/") {
                return Some(SecurityViolation::PathTraversal(arg.clone()));
            }
        }

        // Check for dangerous patterns in command names
        if action.command.contains(';')
            || action.command.contains('|')
            || action.command.contains('&')
            || action.command.contains('$')
            || action.command.contains('`')
        {
            return Some(SecurityViolation::DangerousPattern(action.command.clone()));
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_action(cmd: &str, args: Vec<&str>) -> ValidatedAction {
        ValidatedAction {
            command: cmd.to_string(),
            args: args.into_iter().map(|s| s.to_string()).collect(),
        }
    }

    #[test]
    fn test_path_traversal_detection() {
        let action = create_action("echo", vec!["../../../etc/passwd"]);
        let result = validate_security_rules(&[action]);
        assert!(matches!(result, Some(SecurityViolation::PathTraversal(_))));
    }

    #[test]
    fn test_dangerous_pattern_in_command() {
        let action = create_action("echo; rm -rf /", vec![]);
        let result = validate_security_rules(&[action]);
        assert!(matches!(
            result,
            Some(SecurityViolation::DangerousPattern(_))
        ));
    }

    #[test]
    fn test_dangerous_pattern_in_args() {
        let action = create_action("echo", vec!["hello; world"]);
        let result = validate_security_rules(&[action]);
        assert!(result.is_none()); // Dangerous pattern in args is allowed (just text)
    }

    #[test]
    fn test_safe_command_passes() {
        let action = create_action("echo", vec!["hello"]);
        let result = validate_security_rules(&[action]);
        assert!(result.is_none());
    }

    // ✅ هذه الاختبارات أصبحت غير ضرورية في Core 1.0 لأن Validator يمنعها أولاً
    // لكن نحتفظ بها كـ توثيق مع تعطيلها مؤقتاً

    #[test]
    #[ignore = "هذا الاختبار لـ forbidden commands يتم في Validator, not in rules"]
    fn test_cat_is_forbidden() {
        // هذا الاختبار معطل - forbidden commands تمنع في Validator
    }

    #[test]
    #[ignore = "هذا الاختبار لـ forbidden commands يتم في Validator, not in rules"]
    fn test_ls_is_forbidden() {
        // هذا الاختبار معطل - forbidden commands تمنع في Validator
    }

    #[test]
    #[ignore = "هذا الاختبار لـ forbidden commands يتم في Validator, not in rules"]
    fn test_forbidden_command_detection() {
        // هذا الاختبار معطل - forbidden commands تمنع في Validator
    }
}
