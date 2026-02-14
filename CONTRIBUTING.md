# Contributing to SEL

Thank you for your interest in contributing to SEL!

## Philosophy

SEL's core guarantees are **non-negotiable**:

1. **Determinism** – Same input always produces same output
2. **Security** – Whitelist-based permission model
3. **Canonical Stability** – Changes to canonical form require major version bump

All contributions must preserve these guarantees.

---

## Before You Start

### Prerequisites

- Rust 1.70 or higher
- Familiarity with Rust ecosystem (cargo, clippy, rustfmt)
- Understanding of SEL's core principles

### Development Setup

\`\`\`bash
# Clone repository
git clone https://github.com/chokriabouzid-star/SEL.git
cd SEL

# Build
cargo build

# Run tests
cargo test --workspace

# Run clippy
cargo clippy --workspace
\`\`\`

---

## How to Contribute

### Reporting Bugs

1. **Check existing issues** before creating new ones
2. **Create a new issue** with:
   - Clear, descriptive title
   - Steps to reproduce
   - Expected vs actual behavior
   - Environment details (OS, Rust version)
   - Minimal reproducible example

### Suggesting Features

1. **Open an issue** with tag \`enhancement\`
2. **Explain the use case** clearly
3. **Describe proposed solution**
4. **Consider impact** on determinism and security

### Submitting Pull Requests

#### 1. Fork and Branch

\`\`\`bash
# Fork on GitHub, then:
git clone https://github.com/YOUR_USERNAME/SEL.git
cd SEL
git checkout -b feature/your-feature-name
\`\`\`

#### 2. Make Changes

**Critical Rules:**

- ❌ **NO randomness** – No \`rand\`, no \`UUID::new_v4()\`, no \`SystemTime::now()\`
- ❌ **NO HashMap** in canonical path – Use \`BTreeMap\` for deterministic ordering
- ❌ **NO floats** – SEL rejects floating-point numbers
- ✅ **Add tests** for all new functionality
- ✅ **Update documentation** if changing public API

#### 3. Test Rigorously

\`\`\`bash
# Run all tests
cargo test --workspace

# Format code
cargo fmt --all

# Check with clippy
cargo clippy --workspace -- -D warnings
\`\`\`

**Your PR must:**
- ✅ Pass all existing tests
- ✅ Add new tests for new features
- ✅ Have zero clippy warnings
- ✅ Be formatted with \`cargo fmt\`

#### 4. Commit

Use conventional commits:

\`\`\`bash
git commit -m "feat: add read command with path validation"
git commit -m "fix: correct canonicalization of nested arrays"
git commit -m "docs: update contributing guidelines"
git commit -m "test: add determinism stress test for new feature"
\`\`\`

#### 5. Push and Create PR

\`\`\`bash
git push origin feature/your-feature-name
\`\`\`

---

## Code Review Criteria

PRs are reviewed for:

### 1. Determinism

**Questions:**
- Does this introduce any randomness?
- Are all data structures deterministically ordered?
- Will this produce identical output across platforms?

### 2. Security

**Questions:**
- Does this weaken the whitelist model?
- Are there path traversal vulnerabilities?
- Are resource limits respected?

### 3. Canonical Stability

**Questions:**
- Does this change canonical form?
- If yes, is this a breaking change requiring major version bump?
- Are existing hashes still valid?

---

## Testing Guidelines

### Unit Tests

Every module must have unit tests:

\`\`\`rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_deterministic_behavior() {
        let result1 = function_under_test(input);
        let result2 = function_under_test(input);
        assert_eq!(result1, result2);
    }
}
\`\`\`

### Determinism Tests

Critical for SEL:

\`\`\`rust
#[test]
fn test_determinism_stress() {
    let input = create_test_mission();
    
    for _ in 0..100 {
        let hash = validate_and_hash(input);
        assert_eq!(hash, EXPECTED_HASH);
    }
}
\`\`\`

---

## Code Style

### Rust Conventions

- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use \`rustfmt\` default settings
- Prefer explicit over implicit
- Document public APIs with \`///\` comments

### Error Handling

\`\`\`rust
// ✅ Use Result for recoverable errors
fn validate(mission: &str) -> Result<ValidatedMission, ValidationError>

// ✅ Use custom error types
#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("Invalid JSON: {0}")]
    InvalidJson(String),
}
\`\`\`

---

## Questions?

- **General questions:** Open a [Discussion](https://github.com/chokriabouzid-star/SEL/discussions)
- **Bug reports:** Open an [Issue](https://github.com/chokriabouzid-star/SEL/issues)
- **Security concerns:** Email chokriabouzid@gmail.com

---

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
