
# Facts JSONL Schema v1.0

**Status**: FROZEN (Commercial v1.0)  
**Last Updated**: 2026-02-07  
**Authority**: Product Specification  

---

## Purpose

Defines the immutable, append-only audit trail produced by SEL.

**Facts are used for**:
- Audit trails (compliance evidence)
- Proof of execution (verification)
- Debugging (failure analysis)
- Replay (determinism verification)

---

## Format Properties

### JSONL (JSON Lines)

- **One JSON object per line**
- **No commas between objects**
- **Each line is valid JSON**
- **UTF-8 encoding**
- **LF line endings** (`\n`, not `\r\n`)
- **No comments** (strict JSON)

### Immutability

- **Append-only**: Never modified after writing
- **Chronological order**: Events ordered by occurrence
- **No deletions**: Facts永存 (persist forever)

### File Properties

- **Filename**: `facts.jsonl` (default) or user-specified
- **Location**: Same directory as mission or user-specified
- **Permissions**: `0o644` (world-readable by default)
- **Size limit**: None (grows unbounded)

---

## Event Catalog (COMPLETE SET - 9 types)

### 1. `mission_start`

**When**: Before any action executes  
**Purpose**: Establishes mission context

```json
{
  "type": "mission_start",
  "timestamp": "2026-02-07T14:30:00.123456Z",
  "mission_id": "550e8400-e29b-41d4-a716-446655440000",
  "mission_name": "build-and-test",
  "mission_hash": "sha256:a3f5...",
  "workspace_path": "/tmp/sel-a1b2c3d4",
  "sel_version": "1.0.0",
  "host_os": "Linux",
  "host_arch": "x86_64",
  "rust_version": "1.75.0"
}
```

#### Fields

| Field | Type | Description |
|-------|------|-------------|
| `type` | string | Always `"mission_start"` |
| `timestamp` | RFC3339 | UTC timestamp with microseconds |
| `mission_id` | UUIDv4 | Unique mission execution ID |
| `mission_name` | string | From mission.json `name` field |
| `mission_hash` | string | `sha256:hex` of canonicalized mission |
| `workspace_path` | string | Absolute path to workspace |
| `sel_version` | string | SEL runtime version (semver) |
| `host_os` | string | `Linux`, `Windows`, `macOS` |
| `host_arch` | string | `x86_64`, `aarch64`, etc. |
| `rust_version` | string | Rust compiler version |

---

### 2. `validation_result`

**When**: After mission validation, before execution  
**Purpose**: Records validation outcome

```json
{
  "type": "validation_result",
  "timestamp": "2026-02-07T14:30:00.234567Z",
  "success": true,
  "errors": [],
  "warnings": ["Metadata field is opaque and unused"]
}
```

#### Fields

| Field | Type | Description |
|-------|------|-------------|
| `type` | string | Always `"validation_result"` |
| `timestamp` | RFC3339 | Validation completion time |
| `success` | boolean | `true` if all validations passed |
| `errors` | array[string] | List of validation errors (empty if success) |
| `warnings` | array[string] | Non-fatal warnings |

**Note**: If `success=false`, mission stops here. No further events.

---

### 3. `action_start`

**When**: Before each action executes  
**Purpose**: Marks action boundary

```json
{
  "type": "action_start",
  "timestamp": "2026-02-07T14:30:00.345678Z",
  "action_index": 0,
  "action_type": "command",
  "action_hash": "sha256:b4c3..."
}
```

#### Fields

| Field | Type | Description |
|-------|------|-------------|
| `type` | string | Always `"action_start"` |
| `timestamp` | RFC3339 | Action start time |
| `action_index` | integer | Zero-based index in actions array |
| `action_type` | string | One of: `command`, `file_write`, `file_read`, `file_delete`, `directory_create` |
| `action_hash` | string | `sha256:hex` of canonicalized action JSON |

---

### 4. `command_executed`

**When**: After `command` action completes  
**Purpose**: Records command execution details

```json
{
  "type": "command_executed",
  "timestamp": "2026-02-07T14:30:05.456789Z",
  "action_index": 0,
  "command": "cargo build --release",
  "working_directory": ".",
  "exit_code": 0,
  "duration_ms": 5111,
  "stdout_size_bytes": 2048,
  "stderr_size_bytes": 512,
  "stdout_hash": "sha256:c5d4...",
  "stderr_hash": "sha256:d6e5...",
  "truncated": false
}
```

#### Fields

| Field | Type | Description |
|-------|------|-------------|
| `type` | string | Always `"command_executed"` |
| `timestamp` | RFC3339 | Command completion time |
| `action_index` | integer | Matches `action_start` |
| `command` | string | Exact command executed |
| `working_directory` | string | Workspace-relative path |
| `exit_code` | integer | 0-255 (normal), 124 (timeout), 143 (SIGTERM), 137 (SIGKILL) |
| `duration_ms` | integer | Milliseconds from start to end |
| `stdout_size_bytes` | integer | Total bytes written to stdout |
| `stderr_size_bytes` | integer | Total bytes written to stderr |
| `stdout_hash` | string | `sha256:hex` of stdout (full content) |
| `stderr_hash` | string | `sha256:hex` of stderr (full content) |
| `truncated` | boolean | `true` if output > 1MB (hash only, content not logged) |

**Note**: Full stdout/stderr NOT included in facts (only hashes). Content available separately if needed.

---

### 5. `file_operation`

**When**: After `file_write`, `file_read`, `file_delete`, or `directory_create` action  
**Purpose**: Records file system operations

```json
{
  "type": "file_operation",
  "timestamp": "2026-02-07T14:30:05.567890Z",
  "action_index": 1,
  "operation": "write",
  "path": "output/result.txt",
  "success": true,
  "bytes_written": 42,
  "content_hash": "sha256:e7f6...",
  "permissions": 420
}
```

#### Fields

| Field | Type | Description |
|-------|------|-------------|
| `type` | string | Always `"file_operation"` |
| `timestamp` | RFC3339 | Operation completion time |
| `action_index` | integer | Matches `action_start` |
| `operation` | string | `write`, `read`, `delete`, `create_dir` |
| `path` | string | Workspace-relative path |
| `success` | boolean | `true` if operation succeeded |
| `error_message` | string | Error details (if `success=false`) |
| `bytes_written` | integer | For `write` operations |
| `bytes_read` | integer | For `read` operations |
| `content_hash` | string | `sha256:hex` for write/read |
| `permissions` | integer | Octal as decimal (e.g., 420 = 0o644) |

---

### 6. `verification_result`

**When**: After each verification in `verifications` array  
**Purpose**: Records verification outcome

```json
{
  "type": "verification_result",
  "timestamp": "2026-02-07T14:30:05.678901Z",
  "action_index": 0,
  "verification_type": "exit_code",
  "expected": 0,
  "actual": 0,
  "passed": true
}
```

#### Fields

| Field | Type | Description |
|-------|------|-------------|
| `type` | string | Always `"verification_result"` |
| `timestamp` | RFC3339 | Verification time |
| `action_index` | integer | Parent action index |
| `verification_type` | string | One of 8 verification types |
| `expected` | any | Expected value (type varies) |
| `actual` | any | Actual value observed |
| `passed` | boolean | `true` if verification passed |

**Note**: If `passed=false`, mission stops immediately (fail-fast).

---

### 7. `action_end`

**When**: After action completes (success or failure)  
**Purpose**: Marks action completion

```json
{
  "type": "action_end",
  "timestamp": "2026-02-07T14:30:05.789012Z",
  "action_index": 0,
  "success": true,
  "duration_ms": 5444
}
```

#### Fields

| Field | Type | Description |
|-------|------|-------------|
| `type` | string | Always `"action_end"` |
| `timestamp` | RFC3339 | Action end time |
| `action_index` | integer | Matches `action_start` |
| `success` | boolean | `true` if action and verifications passed |
| `duration_ms` | integer | Total action duration (including verifications) |
| `error_message` | string | Error details (if `success=false`) |

---

### 8. `mission_error`

**When**: On unexpected mission failure (not verification failure)  
**Purpose**: Records system-level errors

```json
{
  "type": "mission_error",
  "timestamp": "2026-02-07T14:30:06.890123Z",
  "error_type": "WorkspaceCreationFailed",
  "error_message": "Permission denied: /tmp/sel-xyz",
  "stack_trace": "... (if available)"
}
```

#### Fields

| Field | Type | Description |
|-------|------|-------------|
| `type` | string | Always `"mission_error"` |
| `timestamp` | RFC3339 | Error occurrence time |
| `error_type` | string | SEL error type (from error enum) |
| `error_message` | string | Human-readable error description |
| `stack_trace` | string | Optional debug information |

**Note**: This is for SEL runtime errors, not mission verification failures.

---

### 9. `mission_end`

**When**: After all actions complete or on mission failure  
**Purpose**: Finalizes mission execution record

```json
{
  "type": "mission_end",
  "timestamp": "2026-02-07T14:30:10.901234Z",
  "mission_id": "550e8400-e29b-41d4-a716-446655440000",
  "success": true,
  "total_duration_ms": 10778,
  "actions_completed": 4,
  "actions_failed": 0,
  "workspace_preserved": false,
  "facts_hash": "sha256:f8g7h6i5j4k3..."
}
```

#### Fields

| Field | Type | Description |
|-------|------|-------------|
| `type` | string | Always `"mission_end"` |
| `timestamp` | RFC3339 | Mission end time |
| `mission_id` | UUIDv4 | Matches `mission_start` |
| `success` | boolean | `true` if all actions succeeded |
| `total_duration_ms` | integer | From mission_start to mission_end |
| `actions_completed` | integer | Number of actions that ran |
| `actions_failed` | integer | Number of failed actions |
| `workspace_preserved` | boolean | `true` if workspace NOT deleted |
| `facts_hash` | string | Cryptographic hash chain of all facts |

---

## Hash Chain Integrity

### Purpose

Enables **tamper detection**: Any modification to facts breaks the chain.

### Algorithm

```
facts_hash = SHA256(
  SHA256(fact_1) ||
  SHA256(fact_2) ||
  ...
  SHA256(fact_N-1)
)
```

Where `||` is concatenation and `fact_N` is `mission_end` event (hash computed before adding `facts_hash` field).

### Verification

Given `facts.jsonl`:

1. Parse all facts (except `mission_end`)
2. Hash each fact's JSON (canonical form)
3. Concatenate hashes
4. Hash the concatenation
5. Compare with `facts_hash` in `mission_end`

**If mismatch → Facts have been tampered with**

---

## Canonicalization for Hashing

**Authority**: See `behavior-spec-v1.md` for the complete, binding algorithm.

**Quick Reference**:

Before hashing (for `action_hash`, `mission_hash`, `facts_hash`):

1. **UTF-8 encoding** strictly enforced
2. **Sort all object keys** alphabetically (recursive)
3. **Compact JSON**: No whitespace, newlines
4. **Unicode NFC normalization**
5. **Consistent number formatting**: No trailing zeros
6. **Minimal string escaping**: Only required escapes

**Example**:

Original event:
```json
{
  "timestamp": "2026-02-07T14:30:00.123456Z",
  "type": "action_start",
  "action_index": 0,
  "action_type": "command"
}
```

Canonicalized (for hashing):
```json
{"action_index":0,"action_type":"command","timestamp":"2026-02-07T14:30:00.123456Z","type":"action_start"}
```

**Hash Calculation**:
```
event_hash = SHA256(canonical_json_utf8_bytes)
```

**Critical**: SEL v1.0.0 reference implementation is the authoritative canonicalizer.
Any deviation produces different hashes (breaks determinism verification).

---

## Size Limits and Truncation

### Per-Event Limits

- **Individual event**: Max 10KB JSON (after formatting)
- **Violations**: Event is still written, but `truncated=true` flag added

### Output Limits

- **stdout/stderr**: Max 1MB per stream
- **If exceeded**: Hash logged, content NOT logged in facts
- **Flag**: `truncated=true` in `command_executed` event

### File Content

- **No limit on file size** for `file_write`/`file_read`
- **Always logged**: Content hash (not content itself)

---

## Retention and Privacy

### Retention Policy

- **Default**: Facts persist forever (user must delete manually)
- **User-controlled**: Retention period configurable (Enterprise)
- **No automatic cleanup**: SEL never deletes facts

### Privacy and PII

- **No PII logged by default**: SEL doesn't extract PII from content
- **User responsibility**: Avoid sensitive data in mission.json
- **Redaction hooks**: Available in Enterprise version

### Example PII Risks

❌ **Bad**: Command with password
```json
{"command": "mysql -p MySecretPassword123"}
```

✅ **Good**: Command with environment variable
```json
{
  "command": "mysql -p $DB_PASSWORD",
  "environment": {"DB_PASSWORD": "<redacted in facts>"}
}
```

---

## Complete Example

Full facts.jsonl for a simple mission:

```jsonl
{"type":"mission_start","timestamp":"2026-02-07T14:30:00.000000Z","mission_id":"a1b2c3d4","mission_name":"hello","mission_hash":"sha256:abc123","workspace_path":"/tmp/sel-xyz","sel_version":"1.0.0","host_os":"Linux","host_arch":"x86_64","rust_version":"1.75.0"}
{"type":"validation_result","timestamp":"2026-02-07T14:30:00.100000Z","success":true,"errors":[],"warnings":[]}
{"type":"action_start","timestamp":"2026-02-07T14:30:00.200000Z","action_index":0,"action_type":"command","action_hash":"sha256:def456"}
{"type":"command_executed","timestamp":"2026-02-07T14:30:00.300000Z","action_index":0,"command":"echo hello","working_directory":".","exit_code":0,"duration_ms":100,"stdout_size_bytes":6,"stderr_size_bytes":0,"stdout_hash":"sha256:ghi789","stderr_hash":"sha256:000000","truncated":false}
{"type":"verification_result","timestamp":"2026-02-07T14:30:00.400000Z","action_index":0,"verification_type":"exit_code","expected":0,"actual":0,"passed":true}
{"type":"action_end","timestamp":"2026-02-07T14:30:00.500000Z","action_index":0,"success":true,"duration_ms":300}
{"type":"mission_end","timestamp":"2026-02-07T14:30:00.600000Z","mission_id":"a1b2c3d4","success":true,"total_duration_ms":600,"actions_completed":1,"actions_failed":0,"workspace_preserved":false,"facts_hash":"sha256:jkl012"}
```

---

## Common Patterns

### Successful Mission

```
mission_start
  → validation_result (success=true)
  → action_start (index=0)
    → command_executed / file_operation
    → verification_result (passed=true) [if any]
    → action_end (success=true)
  → action_start (index=1)
    → ...
  → mission_end (success=true)
```

### Failed Verification

```
mission_start
  → validation_result (success=true)
  → action_start (index=0)
    → command_executed
    → verification_result (passed=false) ← FAIL
    → action_end (success=false)
  → mission_end (success=false)
```

### Failed Action (no verification)

```
mission_start
  → validation_result (success=true)
  → action_start (index=0)
    → file_operation (success=false) ← FAIL
    → action_end (success=false)
  → mission_end (success=false)
```

### Mission Error

```
mission_start
  → validation_result (success=true)
  → action_start (index=0)
    → mission_error ← UNEXPECTED ERROR
  → mission_end (success=false)
```

---

**Document Version**: 1.0.0  
**Status**: FROZEN  
**Changes**: Require v2.0 (breaking)  
**Maintained By**: SEL Product Team


