//! # Security Rules Engine
//! SEL Core 1.0 - Path Traversal & Command Validation

use crate::types::ValidatedAction;

/// List of commands that are strictly forbidden in SEL Core 1.0
/// These commands are blocked regardless of context
const FORBIDDEN_COMMANDS: &[&str] = &[
    "rm", "mv", "cp", "chmod", "chown",          // File operations
    "sudo", "su",                                 // Privilege escalation
    "bash", "sh", "zsh", "fish",                  // Shells
    "python", "python3", "perl", "ruby", "node",  // Interpreters
    "eval", "exec", "system", "popen",            // Code execution
    "spawn", "fork", "clone",                      // Process creation
    "kill", "pkill", "pgrep",                      // Process control
    "cat", "ls", "find", "grep",                   // File reading (allowed in Extended, not Core)
    "wget", "curl", "nc", "netcat",                // Network
];

/// Dangerous patterns in command names or arguments
/// These patterns indicate potential shell injection attempts
const DANGEROUS_PATTERNS: &[&str] = &[
    ";", "|", "&", "$", "`", "\\", "\"", "'",     // Shell injection
];

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
        // 🔴 FIRST CHECK: Command is in forbidden list?
        if FORBIDDEN_COMMANDS.contains(&action.command.as_str()) {
            return Some(SecurityViolation::ForbiddenCommand(action.command.clone()));
        }
        
        // 🔴 SECOND CHECK: Command contains dangerous patterns?
        for pattern in DANGEROUS_PATTERNS {
            if action.command.contains(pattern) {
                return Some(SecurityViolation::DangerousPattern(pattern.to_string()));
            }
        }
        
        // 🔴 THIRD CHECK: Args contain dangerous patterns?
        for arg in &action.args {
            for pattern in DANGEROUS_PATTERNS {
                if arg.contains(pattern) {
                    return Some(SecurityViolation::DangerousPattern(pattern.to_string()));
                }
            }
        }
        
        // 🔴 FOURTH CHECK: Path traversal in args
        for arg in &action.args {
            if arg.contains("..") || arg.starts_with('/') || arg.starts_with("~/") {
                return Some(SecurityViolation::PathTraversal(arg.clone()));
            }
        }
    }
    
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_forbidden_command_detection() {
        let action = ValidatedAction {
            command: "rm".to_string(),
            args: vec!["-rf".to_string(), "/".to_string()],
        };
        
        let result = validate_security_rules(&[action]);
        assert!(matches!(result, Some(SecurityViolation::ForbiddenCommand(cmd)) if cmd == "rm"));
    }
    
    #[test]
    fn test_cat_is_forbidden() {
        let action = ValidatedAction {
            command: "cat".to_string(),
            args: vec!["file.txt".to_string()],
        };
        
        let result = validate_security_rules(&[action]);
        assert!(matches!(result, Some(SecurityViolation::ForbiddenCommand(cmd)) if cmd == "cat"));
    }
    
    #[test]
    fn test_ls_is_forbidden() {
        let action = ValidatedAction {
            command: "ls".to_string(),
            args: vec!["-la".to_string()],
        };
        
        let result = validate_security_rules(&[action]);
        assert!(matches!(result, Some(SecurityViolation::ForbiddenCommand(cmd)) if cmd == "ls"));
    }
    
    #[test]
    fn test_path_traversal_detection() {
        let action = ValidatedAction {
            command: "echo".to_string(),
            args: vec!["../../../etc/passwd".to_string()],
        };
        
        let result = validate_security_rules(&[action]);
        assert!(matches!(result, Some(SecurityViolation::PathTraversal(path)) if path.contains("..")));
    }
    
    #[test]
    fn test_dangerous_pattern_in_args() {
        let action = ValidatedAction {
            command: "echo".to_string(),
            args: vec!["hello".to_string(), "; rm -rf /".to_string()],
        };
        
        let result = validate_security_rules(&[action]);
        assert!(matches!(result, Some(SecurityViolation::DangerousPattern(p)) if p == ";"));
    }
    
    #[test]
    fn test_dangerous_pattern_in_command() {
        let action = ValidatedAction {
            command: "echo;rm".to_string(),
            args: vec!["hello".to_string()],
        };
        
        let result = validate_security_rules(&[action]);
        assert!(matches!(result, Some(SecurityViolation::DangerousPattern(p)) if p == ";"));
    }
    
    #[test]
    fn test_safe_command_passes() {
        let action = ValidatedAction {
            command: "echo".to_string(),
            args: vec!["hello".to_string()],
        };
        
        let result = validate_security_rules(&[action]);
        assert!(result.is_none());
    }
}
