# Behavior Specification v1.0

**Status**: FROZEN (Commercial v1.0)
**Last Updated**: 2026-02-07
**Authority**: Product Specification

---

## 1. Determinism Guarantee (Mathematical)

### Formal Definition
Given:
M₁ = Mission JSON (canonical form)
S₁ = Initial workspace state
E₁ = Execution environment

R₁ = SEL(M₁, S₁, E₁) → Facts F₁

Then for identical inputs:
M₂ = M₁ (bit-identical)
S₂ = S₁ (bit-identical)
E₂ = E₁ (same OS/SEL/arch)

We GUARANTEE:
R₂ = SEL(M₂, S₂, E₂) → Facts F₂
Where: F₂ = F₁ (bit-identical)

text

### Mission Canonicalization (MANDATORY)

**Purpose**: Ensure identical missions produce identical hashes.

**Algorithm** (binding specification):

1. **JSON Encoding**: UTF-8 strictly enforced
2. **Object Keys**: Sorted lexicographically (recursive, all levels)
3. **Whitespace**: Remove all insignificant whitespace
4. **Numbers**: No trailing zeros, no exponential notation
5. **Strings**: Minimal escaping (only `\n`, `\t`, `\"`, `\\`)
6. **Unicode**: Normalized to NFC (Canonical Composition)
7. **Metadata**: Included verbatim in canonicalization (affects hash)
8. **Arrays**: Order preserved (not sorted)
9. **Booleans**: Lowercase `true`/`false`
10. **Null**: Lowercase `null`

**Example**:

Original:
```json
{
  "metadata": {"author": "user"},
  "name":     "test",
  "actions": [
    {
      "timeout_seconds": 3600,
      "type":            "command",
      "command":         "echo hello"
    }
  ]
}
Canonical form:

json
{"actions":[{"command":"echo hello","timeout_seconds":3600,"type":"command"}],"metadata":{"author":"user"},"name":"test"}
Reference Implementation: SEL v1.0.0 canonicalizer is authoritative
Hash Function: SHA-256 of canonical UTF-8 bytes

Environment Normalization (Enforced)
SEL automatically enforces these environment settings for all commands:

Mandatory Variables (always set, user cannot override):

LANG=C.UTF-8 (consistent locale, UTF-8 encoding)

LC_ALL=C.UTF-8 (consistent collation, sorting)

TZ=UTC (timezone always UTC)

PATH=/usr/local/bin:/usr/bin:/bin (predictable path)

Process Properties (enforced by SEL):

umask=0o022 (predictable file creation masks)

Working directory: Workspace root (always)

stdin: /dev/null (no interactive input)

Status: FROZEN
Maintained By: SEL Product Team
