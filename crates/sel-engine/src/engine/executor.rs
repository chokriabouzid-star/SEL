//! # Sovereign Mission Executor
//! SEL Core 1.0 - DETERMINISTIC + NO RANDOMNESS

use sel_common::{ResourceKind, SelResult, SovereignError};
use sel_validator::ValidatedMission;

use super::{
    ActionResult, ExecutionReport, FactsLogger, LogicalClock, ResourceLimits, Workspace,
    WorkspaceMode,
};

pub struct MissionExecutor {
    pub workspace: Workspace,
    pub logical_clock: LogicalClock,
    pub facts_logger: FactsLogger,
    pub limits: ResourceLimits,
    pub mode: WorkspaceMode,
    stdout_bytes_accumulated: usize,
    stderr_bytes_accumulated: usize,
}

impl MissionExecutor {
    pub fn new(mode: WorkspaceMode, mission_hash: &str) -> SelResult<Self> {
        Self::new_with_limits(mode, mission_hash, ResourceLimits::core_compliant())
    }

    pub fn new_with_limits(
        mode: WorkspaceMode,
        mission_hash: &str,
        limits: ResourceLimits,
    ) -> SelResult<Self> {
        // 🔴🔴🔴 PASS MISSION HASH FOR DETERMINISTIC UUID
        let workspace = Workspace::new(mode, mission_hash)?;
        let logical_clock = LogicalClock::new();

        let facts_path = workspace.path().join("facts.jsonl");

        if let Some(parent) = facts_path.parent() {
            if !parent.exists() {
                return Err(SovereignError::WorkspaceCreationFailed(format!(
                    "Workspace directory does not exist: {}",
                    parent.display()
                )));
            }
        }

        let facts_logger = FactsLogger::new(&facts_path).map_err(|e| {
            SovereignError::InternalError(format!(
                "Failed to create facts logger at {}: {}",
                facts_path.display(),
                e
            ))
        })?;

        Ok(Self {
            workspace,
            logical_clock,
            facts_logger,
            limits,
            mode,
            stdout_bytes_accumulated: 0,
            stderr_bytes_accumulated: 0,
        })
    }

    /// Commands permitted at execution time — defense-in-depth mirror of
    /// the validator's own whitelist. Even if a `ValidatedMission` is forged
    /// (all fields are currently `pub` + `Deserialize`), this check ensures
    /// only safe builtins can actually execute. This is a stop-gap until
    /// `VerifiedMission` (re-verification before execution) ships in v1.3.0.
    const EXECUTE_ALLOWED_COMMANDS: &'static [&'static str] = &["echo", "pwd"];

    pub fn execute(&mut self, validated: ValidatedMission) -> SelResult<ExecutionReport> {
        if validated.validation_proof_str().is_empty() {
            return Err(SovereignError::MissingValidationProof);
        }

        let actions = validated.actions();

        // Defense-in-depth: re-check command whitelist at execution time.
        // Validator already enforces this, but ValidatedMission's pub fields
        // and Deserialize impl mean execute() cannot trust the struct was
        // produced by a real Validator. This block catches forged missions.
        for action in &actions {
            if !Self::EXECUTE_ALLOWED_COMMANDS.contains(&action.command.as_str()) {
                return Err(SovereignError::CapabilityViolation(format!(
                    "Command '{}' not in executor whitelist (defense-in-depth).                      Only {:?} are permitted.",
                    action.command,
                    Self::EXECUTE_ALLOWED_COMMANDS
                )));
            }
        }

        if actions.len() > self.limits.max_actions {
            return Err(SovereignError::ResourceExhaustion {
                kind: ResourceKind::Actions,
                limit: self.limits.max_actions as u64,
                requested: actions.len() as u64,
            });
        }

        self.logical_clock.tick();
        let mission_start = serde_json::json!({
            "type": "mission_start",
            "mission_hash": validated.mission_hash(),
            "logical_tick": self.logical_clock.ticks(),
        });

        self.check_facts_limit()?;
        self.facts_logger.log_fact(mission_start)?;

        let mut actions_succeeded = 0;
        let mut actions_failed = 0;
        let start_ticks = self.logical_clock.ticks();

        for (index, action) in actions.iter().enumerate() {
            if self.logical_clock.ticks() + 2 >= self.limits.max_ticks {
                return Err(SovereignError::ResourceExhaustion {
                    kind: ResourceKind::Ticks,
                    limit: self.limits.max_ticks,
                    requested: self.logical_clock.ticks() + 2,
                });
            }

            self.logical_clock.tick();
            let action_start = serde_json::json!({
                "type": "action_start",
                "action_index": index,
                "command": action.command,
                "args": action.args,
                "logical_tick": self.logical_clock.ticks(),
            });

            self.check_facts_limit()?;
            self.facts_logger.log_fact(action_start)?;

            let result = self.execute_builtin(&action.command, &action.args);

            self.stdout_bytes_accumulated += result.stdout.len();
            if self.stdout_bytes_accumulated > self.limits.max_stdout_bytes {
                return Err(SovereignError::ResourceExhaustion {
                    kind: ResourceKind::Stdout,
                    limit: self.limits.max_stdout_bytes as u64,
                    requested: self.stdout_bytes_accumulated as u64,
                });
            }

            self.stderr_bytes_accumulated += result.stderr.len();
            if self.stderr_bytes_accumulated > self.limits.max_stderr_bytes {
                return Err(SovereignError::ResourceExhaustion {
                    kind: ResourceKind::Stderr,
                    limit: self.limits.max_stderr_bytes as u64,
                    requested: self.stderr_bytes_accumulated as u64,
                });
            }

            if result.exit_code == 0 {
                actions_succeeded += 1;
            } else {
                actions_failed += 1;
            }

            self.logical_clock.tick();
            let action_end = serde_json::json!({
                "type": "action_end",
                "action_index": index,
                "exit_code": result.exit_code,
                "stdout_bytes": result.stdout.len(),
                "stderr_bytes": result.stderr.len(),
                "logical_tick": self.logical_clock.ticks(),
            });

            self.check_facts_limit()?;
            self.facts_logger.log_fact(action_end)?;
        }

        self.logical_clock.tick();
        let mission_end = serde_json::json!({
            "type": "mission_end",
            "actions_total": actions.len(),
            "actions_succeeded": actions_succeeded,
            "actions_failed": actions_failed,
            "logical_tick": self.logical_clock.ticks(),
        });

        self.check_facts_limit()?;
        self.facts_logger.log_fact(mission_end)?;

        let final_hash = self.facts_logger.finalize();

        Ok(ExecutionReport {
            mission_hash: validated.mission_hash(),
            validation_proof: validated.validation_proof_str().to_string(),
            validator_version: validated.validator_version().to_string(),
            workspace_uuid: self.workspace.uuid().to_string(),
            actions_total: actions.len(),
            actions_succeeded,
            actions_failed,
            total_duration_ms: 0,
            facts_file: self.facts_logger.path().to_path_buf(),
            final_hash,
            logical_ticks: self.logical_clock.ticks() - start_ticks,
            workspace_mode: format!("{:?}", self.mode),
            stdout_bytes: self.stdout_bytes_accumulated,
            stderr_bytes: self.stderr_bytes_accumulated,
        })
    }

    /// Check `max_facts` limit before every `log_fact` call.
    fn check_facts_limit(&self) -> SelResult<()> {
        let logged = self.facts_logger.fact_count();
        if logged >= self.limits.max_facts {
            return Err(SovereignError::ResourceExhaustion {
                kind: ResourceKind::Facts,
                limit: self.limits.max_facts as u64,
                requested: (logged + 1) as u64,
            });
        }
        Ok(())
    }

    fn execute_builtin(&self, command: &str, args: &[String]) -> ActionResult {
        match command {
            "echo" => {
                let output = args.join(" ");
                ActionResult {
                    exit_code: 0,
                    stdout: output + "\n",
                    stderr: String::new(),
                    duration_ms: 0,
                    action_index: 0,
                }
            }
            "pwd" => ActionResult {
                exit_code: 0,
                stdout: ".\n".to_string(),
                stderr: String::new(),
                duration_ms: 0,
                action_index: 0,
            },
            _ => ActionResult {
                exit_code: 127,
                stdout: String::new(),
                stderr: format!("builtin: command not found: {}\n", command),
                duration_ms: 0,
                action_index: 0,
            },
        }
    }
}
