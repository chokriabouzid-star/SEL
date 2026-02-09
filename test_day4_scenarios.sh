#!/bin/bash
set -e

echo "🧪 Day 4 Test Scenarios"
echo "======================="
echo ""

cd ~/sel-production/SEL

# Scenario 1: Valid mission
echo "1. Testing valid mission..."
cat > /tmp/valid_mission.json << 'JSON'
{
  "name": "valid-test",
  "actions": [
    {"type": "command", "command": "echo", "args": ["SEL Day 4 works!"]}
  ]
}
JSON

cargo run --release --bin sel-engine -- validate /tmp/valid_mission.json && echo "  ✅ Passed" || echo "  ❌ Failed"

# Scenario 2: Forbidden command
echo ""
echo "2. Testing forbidden command (rm)..."
cat > /tmp/forbidden_mission.json << 'JSON'
{
  "name": "forbidden-test",
  "actions": [
    {"type": "command", "command": "rm", "args": ["-rf", "/"]}
  ]
}
JSON

cargo run --release --bin sel-engine -- validate /tmp/forbidden_mission.json 2>&1 | grep -q "Forbidden" && echo "  ✅ Correctly rejected" || echo "  ❌ Should reject"

# Scenario 3: Path traversal
echo ""
echo "3. Testing path traversal attack..."
cat > /tmp/path_escape.json << 'JSON'
{
  "name": "path-escape-test",
  "actions": [
    {"type": "file_write", "path": "../../etc/passwd"}
  ]
}
JSON

cargo run --release --bin sel-engine -- validate /tmp/path_escape.json 2>&1 | grep -q "traversal" && echo "  ✅ Correctly blocked" || echo "  ❌ Should block"

echo ""
echo "✅ All scenarios tested"
