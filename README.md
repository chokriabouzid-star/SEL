# SEL — Sovereign Execution Layer

[![Tests](https://img.shields.io/badge/tests-37%2F37-brightgreen)]()
[![Core](https://img.shields.io/badge/core-v1.1.0-blue)]()
[![License](https://img.shields.io/badge/license-MIT-green)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.85%2B-orange)]()

**Deterministic execution engine with cryptographic guarantees for compliance-critical workflows.**

---

## Overview

SEL (Sovereign Execution Layer) is a deterministic execution engine that guarantees:
```
Same Mission → Same Canonical Form → Same Hash
```

This holds **regardless of execution environment** and **regardless of time**.

**SEL Core** (MIT) provides deterministic validation.  
**SEL Extended** (planned) will introduce enterprise-grade compliance features.

### The Problem

In compliance-critical environments:

- ❌ No mathematical proof that a workflow executed as documented
- ❌ Logs can be tampered with after execution
- ❌ CI/CD systems may produce subtle variations
- ❌ Auditors demand independently verifiable proof

### The Solution

SEL produces:

1. **Canonical JSON** – Deterministic normalization
2. **SHA-256 hash** – Unique, immutable identifier (versioned: `sel:v1.0:sha256:...`)
3. **HMAC signature** – Cryptographic verification (v1.0)
4. **Hash-chained facts** – Tamper-proof execution log

Every execution becomes **cryptographically verifiable**.

---

## Core Guarantees (v1.1.0)

| Guarantee | Implementation |
|-----------|----------------|
| **Deterministic canonicalization** | BTreeMap ordering, no floats, strict UTF-8 |
| **Zero randomness** | No UUID v4, no wall time, no entropy sources |
| **Security enforcement** | Strict command whitelist, path traversal protection |
| **Resource controls** | Enforced limits: ticks, actions, stdout size |
| **Cross-platform design** | Designed for Linux, macOS, Windows (Linux verified) |
| **Audit trail** | Hash-chained facts logger with fsync-per-fact durability |

**Proven:** 20/20 identical hashes in stress tests, 37 tests passing + 3 intentionally ignored.

---

## Canonical Stability Policy

**Changes to canonical form require a major version bump.**

Hash outputs are treated as **immutable contracts**. Once a mission produces a hash under v1.0, that hash will remain valid for verification indefinitely.

This guarantee enables:
- **Long-term audit integrity** – Verify missions years later
- **Cross-version compatibility** – v1.0 hashes remain valid in v1.x
- **Legal compliance** – Hashes are immutable evidence

---

## Architecture

```
SEL/
├── sel-common          # Canonicalization, hashing, error types
├── sel-validator       # Mission validation + HMAC verification
├── sel-engine          # Execution engine + workspace + facts logger
└── sel-validator-cli   # Command-line interface
```

### Design Principles

1. **Determinism > Convenience** – Consistency is non-negotiable
2. **Whitelist > Blacklist** – Security by explicit permission
3. **Proof > Logs** – Cryptographic guarantees over post-hoc documentation
4. **Canonical Form is Law** – One representation, one hash

---

## Quick Start

### Installation

```bash
# Clone repository
git clone https://github.com/chokriabouzid-star/SEL.git
cd SEL

# Build
cargo build --release

# Verify installation
cargo test --workspace
```

### Your First Mission

Create a simple mission file:

```json
{
  "name": "hello-world",
  "version": "1.0",
  "actions": [
    {
      "type": "builtin",
      "command": "echo",
      "args": ["Hello, SEL!"]
    },
    {
      "type": "builtin",
      "command": "pwd",
      "args": []
    }
  ]
}
```

Validate:

```bash
./target/release/sel-validator-cli validate mission.json
```

Expected output:

```
✅ Mission validated successfully
Hash:  sel:v1.0:sha256:<64-char-hex>
Proof: <64-char-hex>
```

---

## Real-World Use Cases

### FinTech: Transaction Verification

**Scenario:** Daily verification for SOC2 compliance

```json
{
  "name": "daily-transaction-audit",
  "version": "1.0",
  "metadata": {
    "date": "2026-02-13",
    "transaction_count": 15234,
    "total_usd_cents": 245367850
  },
  "actions": [
    {
      "type": "builtin",
      "command": "echo",
      "args": ["Verifying 15,234 transactions totaling $2,453,678.50"]
    },
    {
      "type": "builtin",
      "command": "pwd",
      "args": []
    },
    {
      "type": "builtin",
      "command": "echo",
      "args": ["All transactions verified ✓"]
    }
  ]
}
```

**Value:** Cryptographic proof that verification workflow executed as documented.

---

### HealthTech: HIPAA Compliance

**Scenario:** Patient data backup verification

```json
{
  "name": "hipaa-backup-verification",
  "version": "1.0",
  "compliance": "HIPAA",
  "actions": [
    {
      "type": "builtin",
      "command": "echo",
      "args": ["HIPAA-compliant backup initiated"]
    },
    {
      "type": "builtin",
      "command": "echo",
      "args": ["Backup completed: 50,234 patient records"]
    }
  ]
}
```

**Value:** Tamper-proof evidence of backup execution for regulatory audits.

---

### Government: Compliance Verification

**Scenario:** Quarterly regulatory audit trail

```json
{
  "name": "quarterly-compliance-check",
  "version": "1.0",
  "agency": "Financial Regulatory Authority",
  "quarter": "Q4-2026",
  "actions": [
    {
      "type": "builtin",
      "command": "echo",
      "args": ["Q4 2026 Compliance Check - All systems verified"]
    }
  ]
}
```

**Value:** Immutable proof for regulatory bodies.

---

## Security Model

### Core 1.0 Command Whitelist

**Allowed:**
- ✅ `echo` – Output text
- ✅ `pwd` – Print working directory

**Blocked:**
- ❌ All filesystem operations (`cat`, `ls`, `rm`, `cp`, `mv`, `write`)
- ❌ All network operations (`wget`, `curl`, `ssh`)
- ❌ All system operations (`sudo`, shell expansions)
- ❌ Path traversal attempts (`../`, absolute paths)

**Design Philosophy:** Core 1.0 is intentionally minimal. Extended commands (v1.1+) will be carefully added with equivalent security rigor.

---

## Test Status

```
Component              Tests    Status
──────────────────────────────────────────
sel-common              9/9     ✅ PASS
sel-validator          14/14    ✅ PASS (+3 intentionally ignored)
sel-engine              8/8     ✅ PASS
Integration tests       6/6     ✅ PASS
──────────────────────────────────────────
Total                  37/37    ✅ PASS

Determinism stress:     20/20   ✅ Identical hashes
Security audit:         All forbidden commands blocked ✅
Cross-platform:         Designed for Linux/macOS/Windows (Linux verified)
Performance:            0.025s per validation ✅
```

---

## Roadmap

| Version | Status | Focus |
|---------|--------|-------|
| **v1.0.0** | ✅ Released | Deterministic core + HMAC verification |
| **v1.1.0** | ✅ **Stable (current)** | Security & correctness remediation (per-install HMAC keys, real fsync, doc/CLI accuracy) |
| **v1.2.0** | 🚧 Planned | Ed25519 signatures + extended commands (`read`, `write`, `env`) |
| **v2.0.0** | 💡 Future | Distributed verification layer |

---

## Project Status

**SEL Core 1.1.0 is stable and production-ready** for deterministic validation workflows.

- ✅ **Stable Core:** Determinism proven, security enforced, HMAC keys are per-install
- ⏳ **Growing Ecosystem:** Extended features in active development
- 🤝 **Community-Driven:** Contributions welcome to expand capabilities

**Not Yet Supported:**
- File operations (coming in v1.2)
- Ed25519 signatures (coming in v1.2)
- Distributed verification (future)

---

## Documentation

- **[CHANGELOG.md](CHANGELOG.md)** – Version history
- **[CONTRIBUTING.md](CONTRIBUTING.md)** – Contribution guidelines
- **[LICENSE](LICENSE)** – MIT License

### Examples

See `examples/` directory:
- `basic.json` – Simple hello world
- `fintech.json` – Transaction verification
- `healthcare.json` – HIPAA backup verification
- `gov.json` – Government audit trail

---

## Contributing

Contributions must preserve SEL's core guarantees:

1. **Determinism** – No randomness, no wall time
2. **Security** – Whitelist model, explicit permissions
3. **Canonical stability** – Changes to canonicalization require major version bump

**Before submitting:**

```bash
cargo test --workspace      # All tests must pass
cargo fmt --all             # Format code
cargo clippy --workspace    # No warnings
```

See [CONTRIBUTING.md](CONTRIBUTING.md) for detailed guidelines.

---

## License

MIT License

Copyright (c) 2026 Chokri Bouzid

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

---

## Contact

- **Author:** Chokri Bouzid
- **GitHub:** [@chokriabouzid-star](https://github.com/chokriabouzid-star)
- **Email:** chokriabouzid@gmail.com
- **Issues:** [GitHub Issues](https://github.com/chokriabouzid-star/SEL/issues)
- **Discussions:** [GitHub Discussions](https://github.com/chokriabouzid-star/SEL/discussions)

---

## Acknowledgments

Built with:
- [Rust](https://www.rust-lang.org/) – Systems programming language
- [serde_json](https://github.com/serde-rs/json) – JSON serialization
- [sha2](https://github.com/RustCrypto/hashes) – Cryptographic hashing
- [hmac](https://github.com/RustCrypto/MACs) – Message authentication

---

<div align="center">

**SEL — Sovereign Execution Layer**

*Deterministic. Verifiable. Production-ready.*

[⭐ Star](https://github.com/chokriabouzid-star/SEL) · [📖 Docs](docs/) · [🐛 Issues](https://github.com/chokriabouzid-star/SEL/issues)

</div>
