# Changelog

All notable changes to SEL will be documented in this file.

## [1.2.1] - 2026-08-14

### Security Fixes

- **[HIGH] Atomic key file creation — eliminated TOCTOU window (F-003)**
  - Previously `persist_key()` created key files with `File::create()` (umask
    permissions, typically 0644) then called `set_permissions(0o600)` afterward
  - Between creation and chmod, any local process could read HMAC or Ed25519
    key material from the world-readable file
  - Fix: `OpenOptions::new().create_new(true).mode(0o600)` on Unix creates the
    file with restricted permissions atomically at `open(2)` time — no window
  - `sync_all()` added after `write_all()` to guard against partial-write
    corruption on crash
  - Applied identically to both `crypto_authority.rs` and `signature.rs`

- **[HIGH] strict_mode encoded in cryptographic proof payload (F-004)**
  - Previously `--no-strict` produced a proof identical in format to a
    strict-mode proof — a verifier could not distinguish the two
  - Fix: signed payload is now `"strict_mode:{true|false}\n{canonical_json}"`
  - Both HMAC and Ed25519 sign this policy-aware payload; strict/no-strict
    proofs are now cryptographically distinct and cannot be confused
  - `ValidatedMission.strict_mode` field carries the value explicitly
  - CLI prints `• Strict Mode: true/false` and emits a stderr WARNING when
    `--no-strict` is used

- **[MEDIUM] Defense-in-depth whitelist in MissionExecutor::execute() (F-001)**
  - `ValidatedMission` has `pub` fields and derives `Deserialize`, so it can
    be constructed without passing through `Validator::validate()`
  - The only admission check was `!proof_str.is_empty()` — any non-empty
    string would pass
  - Fix: `execute()` now re-checks the command whitelist (`["echo", "pwd"]`)
    before any action runs; a forged mission with an unlisted command is
    rejected with `CapabilityViolation`
  - Note: this is a stop-gap. The architectural fix (`VerifiedMission` type
    requiring cryptographic re-verification before execution) is a blocker
    for v1.3.0

### Correctness Fixes

- **[HIGH] Workspace::Drop no longer deletes facts.jsonl (F-002)**
  - `Workspace` previously implemented `Drop` by calling `cleanup()`, which
    called `fs::remove_dir_all()` on the workspace directory — including the
    `facts.jsonl` tamper-evident audit log
  - The project's own `sel-engine/src/main.rs` already acknowledged this with
    the comment "copy workspace path before execution because it will be
    deleted afterward"
  - A log that self-destructs on normal process exit cannot serve as a durable
    audit trail
  - Fix: `impl Drop` removed entirely. `cleanup()` remains as an explicit,
    opt-in method. Callers must invoke it only after confirming the audit log
    has been persisted elsewhere
  - Integration tests updated to call `executor.workspace.cleanup().ok()`
    explicitly after each test run

### Code Quality

- **SEL_VERSION derived from Cargo.toml at compile time (F-011)**
  - `SEL_VERSION` was hardcoded to `"1.1.0-alpha"` while workspace version
    was 1.2.0. This constant is written into every `ValidatedMission.core_version`
    and therefore into every cryptographic proof
  - Fix: `pub const SEL_VERSION: &str = env!("CARGO_PKG_VERSION")`
  - Removed `SEL_CORE_VERSION` and `SEL_EXTENDED_VERSION` (never referenced
    anywhere in the workspace — confirmed via `rg`)

- **Removed 2 dead-code files from sel-engine (F-010)**
  - `engine/mission_executor.rs` (605 B) and `engine/builtin_echo.rs` (827 B)
    were present on disk but not declared in `engine/mod.rs` — rustc never
    compiled them
  - Mirrors the cleanup done in `sel-validator` in v1.2.0 (4 files removed)
    which was scoped to that crate and never extended to `sel-engine`

- **Updated stale module-level comments**
  - `validator.rs` and `types.rs` still said "HMAC only, no Ed25519" after
    Ed25519 was added in v1.2.0

### Documentation

- README "Not Yet Supported" section corrected:
  - Removed "Ed25519 signatures (coming in v1.2)" — shipped in v1.2.0
  - "File operations" correctly updated to "coming in v1.3"
- README Integration tests row: 6/6 → 7/7
- CHANGELOG [1.0.0]: added inline correction note (actual count was 29+3,
  not 33/33 as originally stated)

### Blocker tracked for v1.3.0

- `VerifiedMission` type: a wrapper around `ValidatedMission` constructible
  only via `Validator::reverify()` which recomputes canonical JSON and
  re-checks HMAC/Ed25519 — makes `MissionExecutor::execute()` structurally
  safe without breaking serialization/transport workflows

### Test Coverage

- 45 passed + 3 intentionally ignored (unchanged from v1.2.0)
- Tests updated to call `workspace.cleanup()` explicitly after each run
  (consequence of F-002 fix: lifecycle management is now the caller's
  responsibility)

---

## [1.2.0] - 2026-08-12

### New Features

- **Ed25519 dual-signing and independent verification**
  - Every validated mission now carries two proofs:
    - HMAC-SHA256 (unchanged from v1.1.0 — existing consumers unaffected)
    - Ed25519 signature over the same canonical JSON
  - `Ed25519Authority`: signing side, key resolution identical to HMAC
    (`SEL_ED25519_KEY_HEX` env var → `~/.sel/ed25519.key` → generated)
  - `Ed25519Verifier`: public-key-only verification — no shared secret
  - New CLI subcommand `verify`:
    ```
    sel-validator-cli verify --mission m.json --signature <hex> --pubkey <hex>
    ```
    Exit 0 = VALID, Exit 1 = INVALID (scriptable in CI)
  - End-to-end verified: `env -i` (empty environment) verify succeeds —
    a true auditor scenario with zero shared secrets

- **`max_facts` limit now enforced in MissionExecutor**
  - Previously `ResourceLimits.max_facts` (default 10,000) was defined
    but never checked during execution
  - `check_facts_limit()` called before every `log_fact()` invocation
  - Returns `ResourceExhaustion { kind: Facts }` when limit is reached
  - Integration test: `test_max_facts_limit_enforced`

### Refactoring

- **Removed 4 dead-code files from sel-validator** (none were reachable
  from the compiler):
  - `types/core.rs` — old Mission/ExecutionPlan design, used `regex` (not in deps)
  - `types/validation.rs` — old ValidationResult/Error
  - `validator/engine.rs` — ValidationEngine built on old types
  - `validator/rules.rs` (10.4 KB) — 18-rule blacklist system, contradicts
    "Whitelist > Blacklist" principle (README Design Principles #2)
  - `signature.rs` kept and rewritten as Ed25519 foundation

### Test Coverage

- ✅ sel-common:          9/9   (unchanged)
- ✅ sel-validator:      21/21  (+7 Ed25519 unit tests)
- ✅ sel-engine:          8/8   (unchanged)
- ✅ integration:         6/6   (+1 max_facts enforcement)
- ⚠️  ignored:            3     (intentional, unchanged)
- ✅ **total: 45 tests passing (was: 37)**
- ✅ Ed25519 end-to-end: validate → verify (empty env) ✅ tamper detection ✅

---

## [1.1.0] - 2026-08-11

### Security Fixes

- **[CRITICAL] Replaced hardcoded HMAC key with per-install key resolution**
  - Previous: `"SEL_CORE_1.0_KEY"` compiled as fixed bytes in open-source code —
    any party with access to the repository could forge valid proofs
  - Now: key resolved at runtime via priority chain:
    1. `SEL_HMAC_KEY_HEX` environment variable (operator/KMS-supplied)
    2. Persisted key at `~/.sel/hmac.key` (auto-created, mode 0600)
    3. Fresh 32-byte random key generated and persisted on first run
  - Fixed test key confined to `#[cfg(test)]` — absent from all release builds
  - `Validator::new()` uses `from_env_or_generate()` (production path only)

- **[CRITICAL] Fixed license metadata conflict**
  - `Cargo.toml` declared `license = "Commercial"` while `LICENSE` file
    contained full MIT text — now consistent: `license = "MIT"`

### Correctness Fixes

- **[HIGH] Implemented real fsync-per-fact durability in FactsLogger**
  - Previous: `log_fact()` called only `flush()` (BufWriter → OS page cache)
    despite documentation claiming "fsync guarantee after each fact"
  - Now: `flush()` + `sync_all()` (OS page cache → physical disk) per fact
  - `FactsLogger::new()` defaults to `durable = true`
  - `FactsLogger::with_durability(path, false)` available for explicit opt-out

- **[HIGH] Fixed CLI output to match README Quick Start specification**
  - Previous output: `VALIDATION SUCCESSFUL` + truncated `Proof: <16 chars>...`
  - Now: `Mission validated successfully` + full `Hash: sel:v1.0:sha256:<hex>`
    + full `Proof: <hex>` — matching documented format exactly
  - Side effect: `scripts/core_test_marathon.sh` hash-extraction grep now works

- **[HIGH] Fixed scripts to run from any directory**
  - Both `core_test_marathon.sh` and `verify_determinism.sh` hardcoded
    `cd ~/sel-production/SEL` — failing immediately on any other machine
  - Now use `BASH_SOURCE[0]`-relative `REPO_ROOT` resolution
  - `core_test_marathon.sh`: added `mktemp` for temp files, `trap` for cleanup

- **[HIGH] Committed Cargo.lock to version control**
  - Previously excluded via `.gitignore` — allowing silent dependency drift
    that breaks the core determinism guarantee
  - Demonstrated real impact: fresh clone with `rustc 1.70.0` (the declared
    MSRV) failed because Cargo resolved `clap_lex v1.1.0` (Edition 2024)
  - `Cargo.lock` is now tracked; update intentionally with `cargo update`

### Test Coverage
- ✅ sel-common: 9/9 tests (unchanged)
- ✅ sel-validator: 14/14 tests (+6 new: HMAC key resolution, persistence, file mode, env var, independence)
- ✅ sel-engine: 8/8 tests (+2 new: durable_by_default, fast_mode_writes_correct_data)
- ✅ integration: 6/6 tests (unchanged)
- ⚠️  ignored: 3 tests (intentional: command rules tested at Validator level, not rules level)
- ✅ **total: 37 tests passing (was: 29)**
- ✅ determinism: 20/20 identical hashes in stress test
- ✅ marathon: 100/100 identical hashes (scripts/core_test_marathon.sh)

text


### Known Remaining Limitations

- `rust-version` updated to `1.85.0` (MSRV): transitive dependencies require
  Edition 2024 support; `1.70.0` was no longer achievable without pinning
  every dependency manually
- `max_facts` limit defined in `ResourceLimits` but not enforced in executor
  (tracked for v1.2.0)
- 5 dead-code files in `sel-validator/src/` (unreachable modules):
  `signature.rs`, `types/core.rs`, `types/validation.rs`,
  `validator/engine.rs`, `validator/rules.rs` — removal planned for v1.2.0
  after evaluating whether to merge their content into the active path
- `examples/full_demo.rs` not wired to any Cargo target (tracked for v1.2.0)
- No Ed25519 signatures (planned for v1.2.0)

---

## [1.0.0] - 2026-02-14

### Initial Stable Release

**SEL Core 1.0.0** – Deterministic execution engine with cryptographic guarantees.

### Features

- **Deterministic Canonicalization**
  - BTreeMap-based key ordering
  - Float rejection (no panic)
  - Contextual path normalization
  - Zero randomness

- **Versioned Hashes**
  - Format: `sel:v1.0:sha256:<hex>`
  - Spec version included in hash input
  - Enables long-term audit integrity

- **Cryptographic Verification**
  - HMAC-SHA256 signatures
  - Deterministic key (same mission = same signature)
  - Clear error messages for tampering

- **Security Enforcement**
  - Strict command whitelist (only `echo`, `pwd`)
  - Path traversal protection (`../`, `/`, `~/`)
  - Forbidden command blocking

- **Resource Limits**
  - `max_actions`: 1000
  - `max_ticks`: 10,000
  - `max_stdout`: 1 MiB
  - Enforced with precise `ResourceKind` errors

- **Audit Trail**
  - Hash-chained facts logger
  - Tamper-proof execution log
  - `fsync` guarantee after each fact

- **Workspace Isolation**
  - Deterministic UUID v5 from mission hash
  - Automatic cleanup on drop
  - Path validation

### Test Status

```
✅ sel-common:          9/9 tests
✅ sel-validator:       8/8 tests + 4 integration
✅ sel-engine:          6/6 tests + 2 integration
✅ total:               33/33 tests passing
⚠️  Correction (see [1.1.0]): actual count at release was 29 passed + 3 ignored; the 33/33 figure was a miscalculation in the original entry. The [1.1.0] entry correctly states 'was: 29'.
✅ determinism:         20/20 identical hashes
✅ security:            All forbidden commands blocked
```

### Documentation

- Professional README with enterprise positioning
- Canonical stability policy
- Real-world use cases (FinTech, HealthTech, Gov)
- Contribution guidelines
- MIT License

### Known Limitations

- No Ed25519 signatures (planned for v1.1)
- Limited commands (only `echo`/`pwd` in Core)
- No file operations (coming in v1.1)
- No distributed verification (future)

### Roadmap

- **v1.1.0** – Ed25519 signatures + extended commands
- **v2.0.0** – Distributed verification layer

---

## [Unreleased]

### Planned

- Ed25519 signatures (dual-crypto)
- Extended commands: `read`, `write`, `env`
- Enhanced workspace operations
- Compliance reporting tools
