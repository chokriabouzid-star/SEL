# Changelog

All notable changes to SEL will be documented in this file.

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
