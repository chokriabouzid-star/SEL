use crate::types::core::Mission;
use crate::types::validation::{ValidationResult, Verdict, Violation};
use crate::validator::rules::*;

pub struct Validator {
    rules: Vec<Box<dyn Rule>>,
}

impl Validator {
    pub fn new() -> Self {
        let mut rules: Vec<Box<dyn Rule>> = Vec::new();
        
        // قواعد الوجود (8 قواعد)
        rules.push(Box::new(MissionHasId));
        rules.push(Box::new(MissionHasVersion));
        rules.push(Box::new(MissionHasExecution));
        rules.push(Box::new(ExecutionHasActions));
        rules.push(Box::new(ActionsNonEmpty));
        rules.push(Box::new(EachActionHasId));
        rules.push(Box::new(EachActionHasCommand));
        rules.push(Box::new(EachActionHasVerification));
        
        // قواعد التطابق (6 قواعد)
        rules.push(Box::new(IdMatchesRegex));
        rules.push(Box::new(VersionMatchesSemver));
        rules.push(Box::new(ActionTypeIsCommand));
        rules.push(Box::new(WorkingDirectoryContainsWorkspaceVar));
        rules.push(Box::new(TimeoutBetween1And3600));
        rules.push(Box::new(CommandLengthUnder1000Chars));
        
        // قواعد السلامة (4 قواعد)
        rules.push(Box::new(NoForbiddenCommands));
        rules.push(Box::new(NoNetworkFlags));
        rules.push(Box::new(NoSystemPaths));
        rules.push(Box::new(NoShellControlChars));
        
        Self { rules }
    }
    
    pub fn validate(&self, mission: &Mission) -> ValidationResult {
        let mut violations = Vec::new();
        let mut rules_passed = 0;
        
        for rule in &self.rules {
            match rule.check(mission) {
                Ok(_) => rules_passed += 1,
                Err((location, fact)) => {
                    violations.push(Violation {
                        rule: rule.name(),
                        location,
                        fact,
                    });
                }
            }
        }
        
        let verdict = if violations.is_empty() {
            Verdict::Valid
        } else {
            Verdict::Invalid
        };
        
        ValidationResult {
            verdict,
            rules_applied: self.rules.len(),
            rules_passed,
            violations,
        }
    }
    
    /// تحقق من Mission JSON مباشرة
    pub fn validate_json(&self, json_str: &str) -> Result<ValidationResult, serde_json::Error> {
        let mission: Mission = serde_json::from_str(json_str)?;
        Ok(self.validate(&mission))
    }
}

impl Default for Validator {
    fn default() -> Self {
        Self::new()
    }
}
