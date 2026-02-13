
# Mission JSON Schema v1.0

**Status**: FROZEN (Commercial v1.0)  
**Last Updated**: 2026-02-07  
**Authority**: Product Specification  

---

## Purpose

Defines the exact structure of a SEL mission. This is a **binding specification**.

Any mission that validates against this schema is **guaranteed to execute** deterministically within the constraints defined in `behavior-spec-v1.md`.

---

## Top-Level Structure

```json
{
  "name": "string",
  "actions": [...],
  "metadata": {...}
}
```

### Field: `name`

- **Type**: `string`
- **Required**: Yes
- **Constraints**:
  - Length: 1-100 characters
  - Pattern: `^[a-zA-Z0-9_-]+$` (alphanumeric, underscore, hyphen only)
  - No whitespace
- **Purpose**: Human-readable mission identifier
- **Example**: `"build-rust-project"`

### Field: `actions`

- **Type**: `array`
- **Required**: Yes
- **Constraints**:
  - Minimum length: 1
  - Maximum length: 1000 (practical limit)
  - All elements must be valid action objects
- **Purpose**: Ordered list of actions to execute

### Field: `metadata`

- **Type**: `object`
- **Required**: No
- **Constraints**: None (completely opaque to SEL)
- **Purpose**: User-defined metadata (not validated, not used by SEL)
- **Note**: SEL never reads or interprets this field

---

## Action Types (EXACTLY 5)

### 1. `command`

Execute a shell command in an isolated subprocess.

```json
{
  "type": "command",
  "command": "cargo build --release",
  "working_directory": "src",
  "timeout_seconds": 600,
  "environment": {
    "RUST_BACKTRACE": "1"
  },
  "verifications": [
    {"type": "exit_code", "expected": 0}
  ]
}
```

#### Fields

| Field | Type | Required | Default | Constraints |
|-------|------|----------|---------|-------------|
| `type` | string | ✅ | - | Must be `"command"` |
| `command` | string | ✅ | - | Non-empty, max 10,000 chars |
| `working_directory` | string | ❌ | `"."` | Workspace-relative path |
| `timeout_seconds` | integer | ❌ | `3600` | 1 to 86400 (24h) |
| `environment` | object | ❌ | `{}` | String key-value pairs |
| `verifications` | array | ❌ | `[]` | See Verifications section |

#### Behavior

- Command executed via `/bin/sh -c` (Linux) or `cmd /C` (Windows)
- Process inherits **NO** environment variables unless explicitly declared
- Network access: **BLOCKED** by default (v1.0)
- Working directory resolved relative to workspace root
- Process killed after timeout with `exit_code = 124`

---

### 2. `file_write`

Write content to a file (atomic operation).

```json
{
  "type": "file_write",
  "path": "output/result.txt",
  "content": "Hello, SEL!",
  "overwrite": false,
  "permissions": 420,
  "verifications": [
    {"type": "file_exists", "path": "output/result.txt"}
  ]
}
```

#### Fields

| Field | Type | Required | Default | Constraints |
|-------|------|----------|---------|-------------|
| `type` | string | ✅ | - | Must be `"file_write"` |
| `path` | string | ✅ | - | Workspace-relative, no `..` |
| `content` | string | ✅ | - | Max 10MB |
| `overwrite` | boolean | ❌ | `false` | - |
| `permissions` | integer | ❌ | `420` (0o644) | Octal as decimal: 0-777 |
| `verifications` | array | ❌ | `[]` | - |

#### Behavior

- Write is **atomic** (temp file + rename)
- If `overwrite=false` and file exists → **FAIL**
- Parent directories created automatically
- Content written **exactly** as provided (UTF-8 encoding assumed)
- Permissions applied after write (Linux/macOS only)

---

### 3. `file_read`

Read a file and optionally verify its content.

```json
{
  "type": "file_read",
  "path": "config.json",
  "expected_pattern": "\"version\":\\s*\"1\\.0\"",
  "fail_if_missing": true,
  "verifications": []
}
```

#### Fields

| Field | Type | Required | Default | Constraints |
|-------|------|----------|---------|-------------|
| `type` | string | ✅ | - | Must be `"file_read"` |
| `path` | string | ✅ | - | Workspace-relative |
| `expected_pattern` | string | ❌ | `null` | Rust regex syntax |
| `fail_if_missing` | boolean | ❌ | `true` | - |
| `verifications` | array | ❌ | `[]` | - |

#### Behavior

- If `fail_if_missing=true` and file missing → **FAIL**
- If `expected_pattern` provided, content must match regex
- File content hashed and logged in facts
- Max file size: 100MB (larger files → hash only, no content logged)

---

### 4. `file_delete`

Delete a file or directory.

```json
{
  "type": "file_delete",
  "path": "temp/cache",
  "recursive": true,
  "fail_if_missing": false,
  "verifications": [
    {"type": "file_not_exists", "path": "temp/cache"}
  ]
}
```

#### Fields

| Field | Type | Required | Default | Constraints |
|-------|------|----------|---------|-------------|
| `type` | string | ✅ | - | Must be `"file_delete"` |
| `path` | string | ✅ | - | Workspace-relative |
| `recursive` | boolean | ❌ | `false` | - |
| `fail_if_missing` | boolean | ❌ | `false` | - |
| `verifications` | array | ❌ | `[]` | - |

#### Behavior

- If `recursive=false` and directory not empty → **FAIL**
- If `fail_if_missing=true` and path missing → **FAIL**
- Deletion is **immediate** (no trash/recycle bin)

---

### 5. `directory_create`

Create a directory (and parents if needed).

```json
{
  "type": "directory_create",
  "path": "build/output",
  "recursive": true,
  "permissions": 493,
  "verifications": [
    {"type": "file_exists", "path": "build/output"}
  ]
}
```

#### Fields

| Field | Type | Required | Default | Constraints |
|-------|------|----------|---------|-------------|
| `type` | string | ✅ | - | Must be `"directory_create"` |
| `path` | string | ✅ | - | Workspace-relative |
| `recursive` | boolean | ❌ | `true` | - |
| `permissions` | integer | ❌ | `493` (0o755) | Octal as decimal |
| `verifications` | array | ❌ | `[]` | - |

#### Behavior

- If path exists (file or directory) → **NO-OP** (success)
- If `recursive=true`, create all parent directories
- Permissions applied to **all** created directories

---

## Verification Criteria (EXACTLY 8)

All actions support an optional `verifications` array:

```json
"verifications": [
  {"type": "...", ...}
]
```

### 1. `exit_code`

**Applies to**: `command` only

```json
{"type": "exit_code", "expected": 0}
```

- **Fields**: `expected` (integer, 0-255)
- **Behavior**: Fails if command exit code ≠ expected

---

### 2. `stdout_contains`

**Applies to**: `command` only

```json
{"type": "stdout_contains", "expected": "success"}
```

- **Fields**: `expected` (string)
- **Behavior**: Fails if stdout does NOT contain substring

---

### 3. `stdout_not_contains`

**Applies to**: `command` only

```json
{"type": "stdout_not_contains", "expected": "error"}
```

- **Fields**: `expected` (string)
- **Behavior**: Fails if stdout DOES contain substring

---

### 4. `stderr_contains`

**Applies to**: `command` only

```json
{"type": "stderr_contains", "expected": "warning"}
```

- **Fields**: `expected` (string)
- **Behavior**: Fails if stderr does NOT contain substring

---

### 5. `stderr_not_contains`

**Applies to**: `command` only

```json
{"type": "stderr_not_contains", "expected": "fatal"}
```

- **Fields**: `expected` (string)
- **Behavior**: Fails if stderr DOES contain substring

---

### 6. `file_exists`

**Applies to**: All action types

```json
{"type": "file_exists", "path": "output.txt"}
```

- **Fields**: `path` (workspace-relative)
- **Behavior**: Fails if file/directory does NOT exist

---

### 7. `file_not_exists`

**Applies to**: All action types

```json
{"type": "file_not_exists", "path": "temp.txt"}
```

- **Fields**: `path` (workspace-relative)
- **Behavior**: Fails if file/directory DOES exist

---

### 8. `file_content_matches`

**Applies to**: All action types

```json
{"type": "file_content_matches", "path": "config.json", "pattern": "^\\{"}
```

- **Fields**: `path`, `pattern` (Rust regex)
- **Behavior**: Fails if file content does NOT match regex

---

## Workspace Management

### Path Resolution

- All paths in `working_directory`, `path`, etc. are **workspace-relative**
- Workspace root is `/tmp/sel-XXXXXX` (Linux) or `%TEMP%\sel-XXXXXX` (Windows)
- Path resolution:
  - `"file.txt"` → `{workspace}/file.txt`
  - `"sub/file.txt"` → `{workspace}/sub/file.txt`
  - `"./file.txt"` → `{workspace}/file.txt`

### Forbidden Paths

- ❌ Absolute paths: `"/etc/passwd"`, `"C:\Windows"`
- ❌ Parent traversal: `"../outside"`, `"sub/../../escape"`
- ❌ Symlinks to outside workspace (followed but validated)

### Workspace Lifecycle

1. Created fresh before mission execution
2. Isolated (no access outside workspace)
3. Cleaned after mission (unless `workspace_preserved=true` in facts)

---

## Mission Canonicalization

Before hashing (for `mission_hash` in facts), missions are canonicalized using the **binding algorithm** defined in `behavior-spec-v1.md`.

### Quick Reference (see behavior-spec for authoritative definition)

1. **UTF-8 encoding** strictly enforced
2. **Sort all object keys** alphabetically (recursive)
3. **Remove insignificant whitespace**
4. **Unicode NFC normalization**
5. **Metadata included** in hash (verbatim)
6. **Default values kept** (do NOT remove in v1.0)
7. **Compact JSON** (no pretty-printing)

### Important Notes

- **Metadata affects hash**: Two missions with different metadata produce different hashes
- **Default values kept**: Unlike earlier specs, v1.0 includes defaults in canonical form
- **Array order preserved**: Actions execute in order, canonicalization preserves order

**Example**:

Original:
```json
{
  "metadata": {"author": "user"},
  "name": "test",
  "actions": [
    {
      "timeout_seconds": 3600,
      "type": "command",
      "command": "echo hello"
    }
  ]
}
```

Canonicalized:
```json
{"actions":[{"command":"echo hello","timeout_seconds":3600,"type":"command"}],"metadata":{"author":"user"},"name":"test"}
```

**Hash Calculation**:
```
mission_hash = "sha256:" + hex(SHA256(canonical_json_bytes))
```

**Purpose**: Same logical mission → same hash, regardless of formatting

**Authority**: SEL v1.0.0 reference implementation is the canonical canonicalizer

---

## Validation Rules (18 Total)

### Schema Validation (8 rules)

1. **REQUIRED_FIELDS**: All required fields present
2. **FIELD_TYPES**: All fields have correct types
3. **NO_UNKNOWN_FIELDS**: No extra fields in mission or actions
4. **NAME_VALID**: Name matches pattern `^[a-zA-Z0-9_-]+$`
5. **ACTIONS_NON_EMPTY**: At least 1 action
6. **ACTION_TYPE_VALID**: Action type is one of 5 allowed
7. **TIMEOUT_RANGE**: Timeout between 1 and 86400
8. **PERMISSIONS_RANGE**: Permissions between 0 and 511 (0o777)

### Path Validation (5 rules)

9. **PATH_RELATIVE**: All paths are workspace-relative
10. **NO_PATH_TRAVERSAL**: No `..` in paths
11. **NO_ABSOLUTE_PATHS**: Paths don't start with `/` or `C:\`
12. **PATH_NOT_EMPTY**: Paths are non-empty strings
13. **WORKING_DIR_VALID**: Working directory is valid relative path

### Content Validation (3 rules)

14. **REGEX_VALID**: All regex patterns compile
15. **CONTENT_SIZE**: File content ≤ 10MB
16. **COMMAND_NON_EMPTY**: Command strings are non-empty

### Logic Validation (2 rules)

17. **VERIFICATION_TYPE_MATCHES**: Verifications match action type
18. **ENV_VARS_VALID**: Environment variable keys are valid identifiers

---

## Complete Example

```json
{
  "name": "build-and-test",
  "actions": [
    {
      "type": "directory_create",
      "path": "build"
    },
    {
      "type": "command",
      "command": "cargo build --release",
      "working_directory": ".",
      "timeout_seconds": 600,
      "environment": {
        "RUST_BACKTRACE": "1"
      },
      "verifications": [
        {"type": "exit_code", "expected": 0},
        {"type": "stdout_contains", "expected": "Finished"},
        {"type": "file_exists", "path": "target/release/myapp"}
      ]
    },
    {
      "type": "command",
      "command": "cargo test",
      "timeout_seconds": 300,
      "verifications": [
        {"type": "exit_code", "expected": 0},
        {"type": "stdout_contains", "expected": "test result: ok"}
      ]
    },
    {
      "type": "file_write",
      "path": "build/SUCCESS",
      "content": "Build completed successfully",
      "verifications": [
        {"type": "file_exists", "path": "build/SUCCESS"}
      ]
    }
  ],
  "metadata": {
    "author": "developer@example.com",
    "ci_job_id": "12345"
  }
}
```

---

## Common Mistakes

### ❌ Absolute paths
```json
{"type": "file_write", "path": "/tmp/output.txt"}  // INVALID
```
✅ **Fix**: Use relative path
```json
{"type": "file_write", "path": "output.txt"}
```

### ❌ Path traversal
```json
{"type": "file_read", "path": "../secrets.txt"}  // INVALID
```
✅ **Fix**: Stay within workspace
```json
{"type": "file_read", "path": "config.txt"}
```

### ❌ Missing required field
```json
{"type": "command"}  // INVALID - missing "command"
```
✅ **Fix**: Add required fields
```json
{"type": "command", "command": "echo hello"}
```

### ❌ Invalid verification for action type
```json
{
  "type": "file_write",
  "path": "out.txt",
  "content": "hello",
  "verifications": [
    {"type": "exit_code", "expected": 0}  // INVALID - file_write has no exit code
  ]
}
```
✅ **Fix**: Use appropriate verification
```json
{
  "type": "file_write",
  "path": "out.txt",
  "content": "hello",
  "verifications": [
    {"type": "file_exists", "path": "out.txt"}
  ]
}
```

---

**Document Version**: 1.0.0  
**Status**: FROZEN  
**Changes**: Require v2.0 (breaking)  
**Maintained By**: SEL Product Team


