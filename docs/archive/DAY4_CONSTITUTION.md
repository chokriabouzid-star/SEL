# Day 4 Constitution - Article I: Authority

**Title:** The Shield - Sovereign Validation Gateway  
**Date:** 2026-02-09  
**Status:** CONSTITUTIONAL SPECIFICATION  
**Version:** 1.0.0

---

## Article I: Philosophical Foundation

### Section 1: Transformation
```yaml
Day 3 Achievement:
  "Proof of Determinism"
  - Same inputs → Same outputs
  - Cryptographic integrity
  - Reproducible execution

Day 4 Transformation:
  "Proof of Authority"
  - What executed ≠ What was PERMITTED to execute
  - Validation is mathematical proof
  - Execution is consequence
  - Logs are evidence
```

### Section 2: Core Principle

**The Sovereignty Axiom:**

> No mission shall execute without cryptographic proof of validation.  
> The language itself enforces this truth.

**In Code:**
```rust
// ❌ Impossible (compile-time error):
executor.execute(raw_mission_string);

// ✅ Only possible path:
let validated = validator.validate(raw)?;
executor.execute(validated);
```

---

## Article II: Type-State Sovereignty

### Section 1: ValidatedMission
```rust
#[derive(Debug, Clone)]
pub struct ValidatedMission {
    /// Original mission (immutable after validation)
    raw: RawMission,
    
    /// Cryptographic attestation (not just hash)
    /// HMAC-SHA256(mission_hash || validator_version || capabilities || workspace_mode)
    /// Future: Can evolve to Ed25519 signature
    validation_proof: String,
    
    /// Validation timestamp (wall clock)
    validation_timestamp: DateTime<Utc>,
    
    /// Validator semantic version
    /// MAJOR.MINOR.PATCH where:
    /// - MAJOR: Schema, capability rules, or path jail logic changed
    /// - MINOR: New validators added (backward compatible)
    /// - PATCH: Bug fixes only
    validator_version: SemanticVersion,
    
    /// Execution capabilities (immutable)
    capabilities: ExecutionCapabilities,
    
    /// Workspace mode (immutable)
    workspace_mode: WorkspaceMode,
}

impl ValidatedMission {
    /// Verify cryptographic proof
    /// 🔴 CRITICAL: Must be called BEFORE any side effects
    pub fn verify_proof(&self) -> Result<(), ProofError> {
        let expected = compute_validation_proof(
            &self.raw,
            &self.validator_version,
            &self.capabilities,
            &self.workspace_mode
        );
        
        if expected != self.validation_proof {
            return Err(ProofError::InvalidSignature {
                expected,
                found: self.validation_proof.clone(),
            });
        }
        
        Ok(())
    }
}
```

### Section 2: ExecutionCapabilities
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExecutionCapabilities {
    /// File system write permission
    can_write_files: bool,
    
    /// Network access permission
    can_read_network: bool,
    
    /// Binary execution permission
    can_execute_binaries: bool,
    
    /// Maximum execution time per action
    max_execution_time: Duration,
    
    /// Maximum memory allocation
    max_memory_mb: u64,
    
    /// Maximum workspace size
    max_workspace_size_mb: u64,
}

impl Default for ExecutionCapabilities {
    fn default() -> Self {
        Self {
            can_write_files: false,        // Deny by default
            can_read_network: false,       // Deny by default
            can_execute_binaries: false,   // Deny by default
            max_execution_time: Duration::from_secs(300),
            max_memory_mb: 512,
            max_workspace_size_mb: 1024,
        }
    }
}
```

### Section 3: WorkspaceMode
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkspaceMode {
    /// Read-only filesystem
    /// 
    /// ⚠️ SECURITY NOTE:
    /// Day 4 implementation uses POSIX permissions (logical RO)
    /// Future (Day 6+): Mount namespace enforcement (kernel RO)
    /// 
    /// Current guarantee: Prevents accidental writes
    /// Future guarantee: Prevents malicious writes
    ReadOnly,
    
    /// Read-write filesystem
    ReadWrite,
}
```

---

## Article III: Validator Architecture

### Section 1: The Gatekeeper
```
┌─────────────────────────────────────────┐
│         Mission Submission              │
└────────────────┬────────────────────────┘
                 │
                 ▼
┌─────────────────────────────────────────┐
│    1. Canonicalization (Day 3)          │
│    → Deterministic form                 │
│    → Mission hash                       │
└────────────────┬────────────────────────┘
                 │
                 ▼
┌─────────────────────────────────────────┐
│    2. VALIDATOR GATE (Day 4)            │
│    ================================     │
│    ┌─────────────────────────────┐     │
│    │ Schema Validation           │     │
│    │ Path Jail Enforcement       │     │
│    │ Command Allowlist Check     │     │
│    │ Capability Computation      │     │
│    │ Cryptographic Signing       │     │
│    └─────────────────────────────┘     │
│                 │                       │
│         ┌───────┴───────┐              │
│         ▼               ▼              │
│    ❌ REJECT       ✅ ACCEPT           │
│    + Errors       + Proof              │
│    + Suggestions  + Capabilities       │
└─────────┬───────────────┬───────────────┘
          │               │
          ▼               ▼
    [Logged]      [Execution Pipeline]
                  (Type-enforced: ValidatedMission only)
```

### Section 2: Validator Interface
```rust
pub struct Validator {
    version: SemanticVersion,
    schema: Schema,
    forbidden_commands: HashSet<String>,
    allowed_paths: Vec<PathBuf>,
    cache: Option<ValidatorCache>,  // Optimization, never authority
}

impl Validator {
    /// Validate mission with caching
    /// 
    /// Cache Policy:
    /// - Key: H(mission_hash || validator_version)
    /// - Miss: Full validation (ALWAYS)
    /// - Hit: Return cached result (optimization only)
    /// - Invalidation: Any validator version change
    pub fn validate(
        &mut self,
        mission: &RawMission
    ) -> ValidationResult {
        let mission_hash = compute_mission_hash(mission);
        
        // Check cache (optimization only)
        if let Some(cached) = self.check_cache(&mission_hash) {
            return ValidationResult::Valid(cached);
        }
        
        // Full validation (authority)
        let result = self.validate_internal(mission)?;
        
        // Cache result
        self.cache_result(&mission_hash, &result);
        
        result
    }
    
    fn validate_internal(
        &self,
        mission: &RawMission
    ) -> Result<ValidatedMission, Vec<ValidationError>> {
        let mut errors = Vec::new();
        
        // 1. Schema validation
        if let Err(e) = self.validate_schema(mission) {
            errors.push(e);
        }
        
        // 2. Path jail
        if let Err(e) = self.validate_paths(mission) {
            errors.push(e);
        }
        
        // 3. Command allowlist
        if let Err(e) = self.validate_commands(mission) {
            errors.push(e);
        }
        
        if !errors.is_empty() {
            return Err(errors);
        }
        
        // 4. Compute capabilities
        let capabilities = self.compute_capabilities(mission);
        
        // 5. Determine workspace mode
        let workspace_mode = self.determine_workspace_mode(mission);
        
        // 6. Sign (cryptographic proof)
        let validation_proof = self.sign(
            mission,
            &capabilities,
            &workspace_mode
        );
        
        Ok(ValidatedMission {
            raw: mission.clone(),
            validation_proof,
            validation_timestamp: Utc::now(),
            validator_version: self.version.clone(),
            capabilities,
            workspace_mode,
        })
    }
    
    fn sign(
        &self,
        mission: &RawMission,
        capabilities: &ExecutionCapabilities,
        workspace_mode: &WorkspaceMode
    ) -> String {
        let mission_hash = compute_mission_hash(mission);
        
        let payload = format!(
            "{}||{}||{}||{}",
            mission_hash,
            self.version,
            serde_json::to_string(capabilities).unwrap(),
            format!("{:?}", workspace_mode)
        );
        
        // Current: HMAC-SHA256
        // Future: Ed25519 signature
        hmac_sha256(&payload, VALIDATOR_SECRET_KEY)
    }
}
```

---

## Article IV: Execution Under Authority

### Section 1: Type-Safe Executor
```rust
impl MissionExecutor {
    /// Execute validated mission
    /// 
    /// 🔴 CRITICAL ORDERING:
    /// 1. Verify proof (FIRST - before ANY side effects)
    /// 2. Setup workspace
    /// 3. Initialize clock
    /// 4. Execute with capabilities
    pub fn execute(
        &mut self,
        validated: ValidatedMission  // ✅ Type-enforced
    ) -> Result<ExecutionReport, ExecutionError> {
        // 🔴 STEP 0: Verify cryptographic proof
        // This MUST be first - before workspace, clock, or any I/O
        validated.verify_proof()
            .map_err(|e| ExecutionError::ProofVerificationFailed(e))?;
        
        // Log validation proof
        self.log_validation_fact(&validated)?;
        
        // Setup workspace with mode
        self.setup_workspace(validated.workspace_mode())?;
        
        // Initialize logical clock
        let mut clock = LogicalClock::new();
        
        // Log mission start
        self.log_mission_start(&validated, &clock)?;
        
        // Execute actions with capability enforcement
        for (idx, action) in validated.raw().actions().iter().enumerate() {
            clock.tick();
            
            self.execute_action(
                idx,
                action,
                validated.capabilities(),
                &clock
            )?;
        }
        
        // Finalize
        let report = self.finalize(&validated, &clock)?;
        
        Ok(report)
    }
    
    fn execute_action(
        &mut self,
        idx: usize,
        action: &Action,
        capabilities: &ExecutionCapabilities,
        clock: &LogicalClock
    ) -> Result<(), ExecutionError> {
        // Log action start
        self.log_action_start(idx, action, clock)?;
        
        // 🔴 ENFORCE CAPABILITIES
        self.enforce_capabilities(action, capabilities)?;
        
        // Execute (sovereign environment)
        let output = self.execute_command_sovereign(
            &action.command,
            &action.args,
            capabilities
        )?;
        
        // Log action end with fsync
        self.log_action_end(idx, action, &output, clock)?;
        self.facts_logger.sync()?;  // 🔴 CRITICAL: fsync after each action
        
        Ok(())
    }
    
    fn enforce_capabilities(
        &self,
        action: &Action,
        capabilities: &ExecutionCapabilities
    ) -> Result<(), CapabilityViolation> {
        // Binary execution check
        if is_binary(&action.command) && !capabilities.can_execute_binaries {
            return Err(CapabilityViolation::BinaryExecutionDenied {
                command: action.command.clone(),
            });
        }
        
        // File write check
        if action.writes_files() && !capabilities.can_write_files {
            return Err(CapabilityViolation::FileWriteDenied {
                action: format!("{:?}", action),
            });
        }
        
        // Network check
        if action.uses_network() && !capabilities.can_read_network {
            return Err(CapabilityViolation::NetworkAccessDenied {
                action: format!("{:?}", action),
            });
        }
        
        Ok(())
    }
}
```

### Section 2: Logical Clock
```rust
#[derive(Debug, Clone)]
pub struct LogicalClock {
    /// Mission epoch (wall clock)
    epoch_start: DateTime<Utc>,
    
    /// Event counter (deterministic)
    ticks: u64,
}

impl LogicalClock {
    pub fn new() -> Self {
        Self {
            epoch_start: Utc::now(),
            ticks: 0,
        }
    }
    
    pub fn tick(&mut self) -> u64 {
        self.ticks += 1;
        self.ticks
    }
    
    /// Logical timestamp (deterministic)
    /// Format: "{epoch}_tick_{counter}"
    pub fn logical_timestamp(&self) -> String {
        format!("{}_tick_{}", 
            self.epoch_start.to_rfc3339(), 
            self.ticks)
    }
    
    /// Wall clock (for reference only)
    pub fn wall_clock(&self) -> DateTime<Utc> {
        Utc::now()
    }
}
```

---

## Article V: Facts Schema

### Section 1: Validation Fact (NEW)
```json
{
  "type": "validation_proof",
  "mission_hash": "sha256:...",
  "validator_version": "1.0.0",
  "validation_proof": "hmac-sha256:...",
  "capabilities": {
    "can_write_files": true,
    "can_read_network": false,
    "can_execute_binaries": false,
    "max_execution_time_secs": 300,
    "max_memory_mb": 512,
    "max_workspace_size_mb": 1024
  },
  "workspace_mode": "ReadWrite",
  "result": "accepted",
  "logical_time": "2026-02-09T10:00:00Z_tick_0",
  "wall_clock": "2026-02-09T10:00:00.123Z"
}
```

**Purpose:** Prove that a mission was validated, even if not executed.

### Section 2: Mission Start Fact (Updated)
```json
{
  "type": "mission_start",
  "mission_hash": "sha256:...",
  "validation_proof": "hmac-sha256:...",
  "validator_version": "1.0.0",
  "capabilities": { /* ... */ },
  "workspace_mode": "ReadWrite",
  "workspace_uuid": "...",
  "logical_time": "2026-02-09T10:00:00Z_tick_1",
  "wall_clock": "2026-02-09T10:00:00.234Z"
}
```

### Section 3: Action Facts (Updated)
```json
{
  "type": "action_start",
  "action_index": 0,
  "command": "echo",
  "logical_time": "2026-02-09T10:00:00Z_tick_2",
  "wall_clock": "2026-02-09T10:00:00.345Z"
}

{
  "type": "action_end",
  "action_index": 0,
  "exit_code": 0,
  "duration_ms": 5,
  "logical_time": "2026-02-09T10:00:00Z_tick_3",
  "wall_clock": "2026-02-09T10:00:00.350Z"
}
```

**Note:** Logical time is deterministic, wall clock is reference only.

---

## Article VI: Explicit Non-Goals

### What Day 4 Does NOT Aim To:
```yaml
❌ NOT a sandbox:
  - SEL is not an OS-level sandbox
  - Does not replace kernel security (SELinux, AppArmor)
  - Does not protect against root attacker

❌ NOT a container:
  - Does not provide namespace isolation (yet)
  - Does not provide cgroup limits (yet)
  - Future: Day 6+ may add these

❌ NOT cryptographically secure storage:
  - Validation proof uses HMAC (symmetric)
  - Future: May upgrade to Ed25519 (asymmetric)
  - Current: Prevents tampering, not forgery by insider

✅ What Day 4 IS:
  - Language-enforced validation gate
  - Capability-based execution
  - Audit trail with cryptographic integrity
  - Deterministic execution with logical clock
```

---

## Article VII: Threat Model

### Threats Mitigated:
```yaml
✅ Accidental policy violations:
  - Unvalidated missions rejected at compile time
  - Capability enforcement prevents mistakes

✅ Path traversal attacks:
  - Path jail enforced by validator
  - Workspace isolation

✅ Privilege escalation:
  - Capabilities immutable after validation
  - Executor cannot grant permissions

✅ Audit trail tampering:
  - Hash chain integrity
  - fsync after each action
  - Validation proof in facts

✅ Non-deterministic execution:
  - Logical clock
  - Canonicalization
  - Environment normalization
```

### Threats NOT Mitigated (Explicit):
```yaml
❌ Malicious validator:
  - If validator is compromised, system trusts it
  - Mitigation: Validator versioning + audit

❌ Kernel exploits:
  - SEL runs in userspace
  - Cannot prevent kernel-level attacks

❌ Hardware attacks:
  - No protection against physical access
  - No TPM/SGX integration (yet)

❌ Side-channel attacks:
  - Timing attacks not addressed
  - Cache attacks not addressed
```

---

## Article VIII: Semantic Versioning Contract

### Validator Version Format: MAJOR.MINOR.PATCH
```yaml
MAJOR (breaking changes):
  - Schema modifications
  - Capability rules changed
  - Path jail logic updated
  - Any change affecting validation outcome

MINOR (backward compatible):
  - New optional validators added
  - Performance improvements
  - Cache optimizations

PATCH (bug fixes only):
  - Error message improvements
  - Documentation updates
  - Test coverage improvements
```

### Version Bump Rules:
```rust
// Example:

// v1.0.0 → v2.0.0 (MAJOR)
// - Added new forbidden command
// - Changes validation outcome

// v1.0.0 → v1.1.0 (MINOR)
// - Added validator cache
// - Same validation outcome

// v1.0.0 → v1.0.1 (PATCH)
// - Fixed error message typo
// - Same validation outcome
```

---

## Article IX: Success Criteria

### Build:
- ✅ All crates compile without warnings
- ✅ Release build < 2MB
- ✅ No unsafe code in critical paths

### Tests:
- ✅ Unit tests: 100% pass
- ✅ Integration tests: All scenarios covered
- ✅ Property tests: Determinism verified

### Security:
- ✅ Type-state enforced at compile time
- ✅ Forbidden commands blocked
- ✅ Path escapes prevented
- ✅ Capabilities enforced
- ✅ Proof verification working

### Performance:
- ✅ Validation < 100ms for typical mission
- ✅ Cache hit rate > 80% in repeated executions
- ✅ fsync overhead < 5% of total execution time

---

## Conclusion

This is not a "Day 4 plan."  
This is **SEL Constitution - Article I: Authority.**

The transformation from execution engine to sovereign trust infrastructure is complete in specification.

**Next:** Implementation.

---

**Signed:** SEL Engineering Team  
**Date:** 2026-02-09  
**Version:** 1.0.0  
**Status:** CONSTITUTIONAL
