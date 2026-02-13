//! # Security Rules Engine
//! SEL Core 1.0 - Path Traversal & Command Validation

use crate::types::ValidatedAction;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SecurityViolation {
    PathTraversal(String),
    ForbiddenCommand(String),
    DangerousPattern(String),
}

/// Validate security rules for actions
/// Returns Some(violation) if rule is broken, None if safe
pub fn validate_security_rules(actions: &[ValidatedAction]) -> Option<SecurityViolation> {
    for action in actions {
        // Check for path traversal in args
        for arg in &action.args {
            if arg.contains("..") {
                return Some(SecurityViolation::PathTraversal(arg.clone()));
            }
            
            if arg.starts_with('/') || arg.starts_with("~/") {
                return Some(SecurityViolation::PathTraversal(arg.clone()));
            }
        }
        
        // Check for forbidden commands
        match action.command.as_str() {
            "rm" | "mv" | "cp" | "chmod" | "chown" | "sudo" | "su" | "bash" | "sh" | "zsh" | "python" | "perl" | "ruby" | "node" | "eval" | "exec" | "system" | "popen" | "spawn" | "fork" | "clone" | "kill" | "pkill" => {
                return Some(SecurityViolation::ForbiddenCommand(action.command.clone()));
            }
            _ => {}
        }
        
        // Check for dangerous patterns in command names
        if action.command.contains(';') || action.command.contains('|') || action.command.contains('&') || action.command.contains('$') || action.command.contains('`') {
            return Some(SecurityViolation::DangerousPattern(action.command.clone()));
        }
    }
    
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_path_traversal_detection() {
        let action = ValidatedAction {
            command: "echo".to_string(),
            args: vec!["../../../etc/passwd".to_string()],
        };
        
        let result = validate_security_rules(&[action]);
        assert!(matches!(result, Some(SecurityViolation::PathTraversal(_))));
    }
    
    #[test]
    fn test_forbidden_command_detection() {
        let action = ValidatedAction {
            command: "rm".to_string(),
            args: vec!["-rf".to_string(), "/".to_string()],
        };
        
        let result = validate_security_rules(&[action]);
        assert!(matches!(result, Some(SecurityViolation::ForbiddenCommand(_))));
    }
}
