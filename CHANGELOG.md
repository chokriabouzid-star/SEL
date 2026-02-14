# Changelog

All notable changes to SEL will be documented in this file.

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
  - Format: \`sel:v1.0:sha256:<hex>\`
  - Spec version included in hash input
  - Enables long-term audit integrity

- **Cryptographic Verification**
  - HMAC-SHA256 signatures
  - Deterministic key (same mission = same signature)
  - Clear error messages for tampering

- **Security Enforcement**
  - Strict command whitelist (only \`echo\`, \`pwd\`)
  - Path traversal protection (\`../\`, \`/\`, \`~/\`)
  - Forbidden command blocking

- **Resource Limits**
  - \`max_actions\`: 1000
  - \`max_ticks\`: 10,000
  - \`max_stdout\`: 1 MiB
  - Enforced with precise \`ResourceKind\` errors

- **Audit Trail**
  - Hash-chained facts logger
  - Tamper-proof execution log
  - \`fsync\` guarantee after each fact

- **Workspace Isolation**
  - Deterministic UUID v5 from mission hash
  - Automatic cleanup on drop
  - Path validation

### Test Status

\`\`\`
✅ sel-common:          9/9 tests
✅ sel-validator:       8/8 tests + 4 integration
✅ sel-engine:          6/6 tests + 2 integration
✅ total:               33/33 tests passing
✅ determinism:         20/20 identical hashes
✅ security:            All forbidden commands blocked
\`\`\`

### Documentation

- Professional README with enterprise positioning
- Canonical stability policy
- Real-world use cases (FinTech, HealthTech, Gov)
- Contribution guidelines
- MIT License

### Known Limitations

- No Ed25519 signatures (planned for v1.1)
- Limited commands (only \`echo\`/\`pwd\` in Core)
- No file operations (coming in v1.1)
- No distributed verification (future)

### Roadmap

- **v1.1.0** – Ed25519 signatures + extended commands
- **v2.0.0** – Distributed verification layer

---

## [Unreleased]

### Planned

- Ed25519 signatures (dual-crypto)
- Extended commands: \`read\`, \`write\`, \`env\`
- Enhanced workspace operations
- Compliance reporting tools
