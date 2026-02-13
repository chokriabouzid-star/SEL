#!/bin/bash
set -e

echo "🏃 SEL Core 1.0 - Testing Marathon"
echo "==================================="

cd ~/sel-production/SEL

# Unit tests
echo "1️⃣ Unit Tests"
cargo test -p sel-common -- --nocapture
cargo test -p sel-validator -- --nocapture
cargo test -p sel-engine -- --nocapture

# Determinism stress test
echo ""
echo "2️⃣ Determinism Stress Test (100 iterations)"
MISSION='{"name":"stress-test","actions":[{"command":"echo","args":["test"]}]}'

# Create a temporary mission file
echo "$MISSION" > /tmp/test-mission.json

FIRST_HASH=""
for i in {1..100}; do
    HASH=$(cargo run -q -p sel-validator-cli -- validate /tmp/test-mission.json 2>/dev/null | grep -o "sel:v1.0:sha256:[a-f0-9]\+" || echo "")
    
    if [ -z "$HASH" ]; then
        echo "❌ Failed to get hash at iteration $i"
        exit 1
    fi
    
    if [ -z "$FIRST_HASH" ]; then
        FIRST_HASH="$HASH"
        echo "  Iteration 1: $FIRST_HASH"
    elif [ "$HASH" != "$FIRST_HASH" ]; then
        echo "❌ Hash mismatch at iteration $i"
        echo "Expected: $FIRST_HASH"
        echo "Got:      $HASH"
        exit 1
    fi
    
    if [ $((i % 10)) -eq 0 ]; then
        echo "  Iteration $i/100: ✓"
    fi
done

echo "✅ All 100 iterations produced identical hash: $FIRST_HASH"
echo ""
echo "=================================="
echo "✅ Core 1.0 Testing Marathon COMPLETE"
echo "=================================="
