use crate::types::core::Mission;
use regex::Regex;
use std::sync::OnceLock;

// استخدام OnceLock بدلاً من lazy_static
fn id_regex() -> &'static Regex {
    static ID_REGEX: OnceLock<Regex> = OnceLock::new();
    ID_REGEX.get_or_init(|| Regex::new(r"^[a-z0-9-]+$").unwrap())
}

fn forbidden_commands() -> &'static [&'static str] {
    &[
        "rm -rf", "sudo", "chmod 777",
        "curl", "wget", "ssh", "nc"
    ]
}

fn network_flags() -> &'static [&'static str] {
    &["--network", "-n", "--fetch"]
}

fn shell_control_chars() -> &'static [&'static str] {
    &["&&", "||", ";", "|"]
}

/// قاعدة التحقق
pub trait Rule {
    fn check(&self, mission: &Mission) -> Result<(), (String, String)>;
    
    fn name(&self) -> String;
}

// ========== قواعد الوجود (8 قواعد) ==========

pub struct MissionHasId;
impl Rule for MissionHasId {
    fn check(&self, mission: &Mission) -> Result<(), (String, String)> {
        if mission.id.is_empty() {
            Err((".id".to_string(), "field is empty".to_string()))
        } else {
            Ok(())
        }
    }
    
    fn name(&self) -> String { "MISSION_HAS_ID".to_string() }
}

pub struct MissionHasVersion;
impl Rule for MissionHasVersion {
    fn check(&self, mission: &Mission) -> Result<(), (String, String)> {
        if mission.version.is_empty() {
            Err((".version".to_string(), "field is empty".to_string()))
        } else {
            Ok(())
        }
    }
    
    fn name(&self) -> String { "MISSION_HAS_VERSION".to_string() }
}

pub struct MissionHasExecution;
impl Rule for MissionHasExecution {
    fn check(&self, _mission: &Mission) -> Result<(), (String, String)> {
        // التحقق من وجود execution فقط، لا محتواه
        Ok(())
    }
    
    fn name(&self) -> String { "MISSION_HAS_EXECUTION".to_string() }
}

pub struct ExecutionHasActions;
impl Rule for ExecutionHasActions {
    fn check(&self, _mission: &Mission) -> Result<(), (String, String)> {
        // التحقق من وجود actions فقط
        Ok(())
    }
    
    fn name(&self) -> String { "EXECUTION_HAS_ACTIONS".to_string() }
}

pub struct ActionsNonEmpty;
impl Rule for ActionsNonEmpty {
    fn check(&self, mission: &Mission) -> Result<(), (String, String)> {
        if mission.execution.actions.is_empty() {
            Err((".execution.actions".to_string(), "array is empty".to_string()))
        } else {
            Ok(())
        }
    }
    
    fn name(&self) -> String { "ACTIONS_NON_EMPTY".to_string() }
}

pub struct EachActionHasId;
impl Rule for EachActionHasId {
    fn check(&self, mission: &Mission) -> Result<(), (String, String)> {
        for (i, action) in mission.execution.actions.iter().enumerate() {
            if action.id == 0 {
                return Err((
                    format!(".execution.actions[{}].id", i),
                    "id must be non-zero".to_string()
                ));
            }
        }
        Ok(())
    }
    
    fn name(&self) -> String { "EACH_ACTION_HAS_ID".to_string() }
}

pub struct EachActionHasCommand;
impl Rule for EachActionHasCommand {
    fn check(&self, mission: &Mission) -> Result<(), (String, String)> {
        for (i, action) in mission.execution.actions.iter().enumerate() {
            if action.command.is_empty() {
                return Err((
                    format!(".execution.actions[{}].command", i),
                    "field is empty".to_string()
                ));
            }
        }
        Ok(())
    }
    
    fn name(&self) -> String { "EACH_ACTION_HAS_COMMAND".to_string() }
}

pub struct EachActionHasVerification;
impl Rule for EachActionHasVerification {
    fn check(&self, _mission: &Mission) -> Result<(), (String, String)> {
        // التحقق من وجود verification فقط
        // لا تحليل لمحتواه
        Ok(())
    }
    
    fn name(&self) -> String { "EACH_ACTION_HAS_VERIFICATION".to_string() }
}

// ========== قواعد التطابق (6 قواعد) ==========

pub struct IdMatchesRegex;
impl Rule for IdMatchesRegex {
    fn check(&self, mission: &Mission) -> Result<(), (String, String)> {
        if !id_regex().is_match(&mission.id) {
            Err((
                ".id".to_string(),
                format!("must match pattern: {}", id_regex().as_str())
            ))
        } else {
            Ok(())
        }
    }
    
    fn name(&self) -> String { "ID_MATCHES_REGEX".to_string() }
}

pub struct VersionMatchesSemver;
impl Rule for VersionMatchesSemver {
    fn check(&self, mission: &Mission) -> Result<(), (String, String)> {
        if semver::Version::parse(&mission.version).is_err() {
            Err((
                ".version".to_string(),
                "must be valid semver".to_string()
            ))
        } else {
            Ok(())
        }
    }
    
    fn name(&self) -> String { "VERSION_MATCHES_SEMVER".to_string() }
}

pub struct ActionTypeIsCommand;
impl Rule for ActionTypeIsCommand {
    fn check(&self, mission: &Mission) -> Result<(), (String, String)> {
        for (i, action) in mission.execution.actions.iter().enumerate() {
            if action.action_type != "command" {
                return Err((
                    format!(".execution.actions[{}].type", i),
                    "must be 'command' (only value allowed in v0.1)".to_string()
                ));
            }
        }
        Ok(())
    }
    
    fn name(&self) -> String { "ACTION_TYPE_IS_COMMAND".to_string() }
}

pub struct WorkingDirectoryContainsWorkspaceVar;
impl Rule for WorkingDirectoryContainsWorkspaceVar {
    fn check(&self, mission: &Mission) -> Result<(), (String, String)> {
        for (i, action) in mission.execution.actions.iter().enumerate() {
            let contains_var = action.working_directory.contains("${workspace}") || 
                               action.working_directory.contains("${mission_id}");
            
            if !contains_var {
                return Err((
                    format!(".execution.actions[{}].working_directory", i),
                    "must contain '${workspace}' or '${mission_id}' variable".to_string()
                ));
            }
        }
        Ok(())
    }
    
    fn name(&self) -> String { "WORKING_DIRECTORY_CONTAINS_WORKSPACE_VAR".to_string() }
}

pub struct TimeoutBetween1And3600;
impl Rule for TimeoutBetween1And3600 {
    fn check(&self, mission: &Mission) -> Result<(), (String, String)> {
        for (i, action) in mission.execution.actions.iter().enumerate() {
            if action.timeout_seconds < 1 || action.timeout_seconds > 3600 {
                return Err((
                    format!(".execution.actions[{}].timeout_seconds", i),
                    "must be between 1 and 3600 seconds".to_string()
                ));
            }
        }
        Ok(())
    }
    
    fn name(&self) -> String { "TIMEOUT_BETWEEN_1_AND_3600".to_string() }
}

pub struct CommandLengthUnder1000Chars;
impl Rule for CommandLengthUnder1000Chars {
    fn check(&self, mission: &Mission) -> Result<(), (String, String)> {
        for (i, action) in mission.execution.actions.iter().enumerate() {
            if action.command.len() > 1000 {
                return Err((
                    format!(".execution.actions[{}].command", i),
                    "length must be under 1000 characters".to_string()
                ));
            }
        }
        Ok(())
    }
    
    fn name(&self) -> String { "COMMAND_LENGTH_UNDER_1000_CHARS".to_string() }
}

// ========== قواعد السلامة (4 قواعد) ==========

pub struct NoForbiddenCommands;
impl Rule for NoForbiddenCommands {
    fn check(&self, mission: &Mission) -> Result<(), (String, String)> {
        for (i, action) in mission.execution.actions.iter().enumerate() {
            for forbidden in forbidden_commands().iter() {
                if action.command.contains(forbidden) {
                    return Err((
                        format!(".execution.actions[{}].command", i),
                        format!("contains forbidden command: '{}'", forbidden)
                    ));
                }
            }
        }
        Ok(())
    }
    
    fn name(&self) -> String { "NO_FORBIDDEN_COMMANDS".to_string() }
}

pub struct NoNetworkFlags;
impl Rule for NoNetworkFlags {
    fn check(&self, mission: &Mission) -> Result<(), (String, String)> {
        for (i, action) in mission.execution.actions.iter().enumerate() {
            for flag in network_flags().iter() {
                if action.command.contains(flag) {
                    return Err((
                        format!(".execution.actions[{}].command", i),
                        format!("contains network flag: '{}'", flag)
                    ));
                }
            }
        }
        Ok(())
    }
    
    fn name(&self) -> String { "NO_NETWORK_FLAGS".to_string() }
}

pub struct NoSystemPaths;
impl Rule for NoSystemPaths {
    fn check(&self, mission: &Mission) -> Result<(), (String, String)> {
        let system_paths = vec!["/bin", "/usr", "/etc", "/home", "/root", "/var", "/sys"];
        for (i, action) in mission.execution.actions.iter().enumerate() {
            for path in &system_paths {
                if action.command.contains(path) || action.working_directory.contains(path) {
                    return Err((
                        format!(".execution.actions[{}]", i),
                        format!("references system path: '{}'", path)
                    ));
                }
            }
        }
        Ok(())
    }
    
    fn name(&self) -> String { "NO_SYSTEM_PATHS".to_string() }
}

pub struct NoShellControlChars;
impl Rule for NoShellControlChars {
    fn check(&self, mission: &Mission) -> Result<(), (String, String)> {
        for (i, action) in mission.execution.actions.iter().enumerate() {
            for control_char in shell_control_chars().iter() {
                if action.command.contains(control_char) {
                    return Err((
                        format!(".execution.actions[{}].command", i),
                        format!("contains shell control character: '{}'", control_char)
                    ));
                }
            }
        }
        Ok(())
    }
    
    fn name(&self) -> String { "NO_SHELL_CONTROL_CHARS".to_string() }
}
