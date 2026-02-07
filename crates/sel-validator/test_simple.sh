#!/bin/bash
echo "=== SEL Validator Quick Test ==="

# بني المشروع
echo "Building..."
cargo build --quiet

# اختبر مباشرة باستخدام Rust
echo "Running quick Rust test..."
cat > /tmp/test_mission.json << 'TESTJSON'
{
  "id": "quick-test",
  "version": "1.0.0",
  "execution": {
    "actions": [{
      "id": 1,
      "type": "command",
      "command": "echo hello",
      "working_directory": "/workspace/${mission_id}",
      "timeout_seconds": 30,
      "verification": {
        "exit_code": 0
      }
    }]
  }
}
TESTJSON

# استخدم cargo run لاختبار مباشر
echo "Test 1: Valid mission"
cargo run --quiet -- /tmp/test_mission.json 2>/dev/null || echo "Exit code: $?"

echo -e "\nTest 2: Invalid mission (no command)"
cat > /tmp/invalid.json << 'INVALID'
{
  "id": "",
  "version": "not-semver",
  "execution": {
    "actions": []
  }
}
INVALID
cargo run --quiet -- /tmp/invalid.json 2>&1 | head -5

echo -e "\nTest 3: Forbidden command"
cat > /tmp/forbidden.json << 'FORBIDDEN'
{
  "id": "test",
  "version": "1.0.0",
  "execution": {
    "actions": [{
      "id": 1,
      "type": "command",
      "command": "sudo rm -rf /",
      "working_directory": "/workspace/${mission_id}",
      "timeout_seconds": 30,
      "verification": {"exit_code": 0}
    }]
  }
}
FORBIDDEN
cargo run --quiet -- /tmp/forbidden.json 2>&1 | grep -E "(VALID|INVALID|Forbidden)"

echo -e "\n=== Test Complete ==="
