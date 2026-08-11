#!/bin/bash
# SEL Core 1.0 - Testing Marathon
# Usage: bash scripts/core_test_marathon.sh (from any directory)
set -e

# Always resolve paths relative to this script's location,
# never rely on a hardcoded absolute path.
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

echo "🏃 SEL Core 1.0 - Testing Marathon"
echo "==================================="
echo "   Repo: $REPO_ROOT"
echo ""

cd "$REPO_ROOT"

# ── 1. Unit tests ────────────────────────────────────────────────────────
echo "1️⃣  Unit Tests"
cargo test -p sel-common    -- --nocapture
cargo test -p sel-validator -- --nocapture
cargo test -p sel-engine    -- --nocapture

# ── 2. Determinism stress test (100 iterations) ──────────────────────────
echo ""
echo "2️⃣  Determinism Stress Test (100 iterations)"

MISSION='{"actions":[{"command":"echo","args":["test"]}]}'
MISSION_FILE="$(mktemp /tmp/sel-stress-XXXXXX.json)"
echo "$MISSION" > "$MISSION_FILE"
trap 'rm -f "$MISSION_FILE"' EXIT

FIRST_HASH=""
for i in $(seq 1 100); do
    # stderr (SEL WARNING on first run) is intentionally kept visible so
    # operators know a key was generated; only the CLI output is filtered.
    HASH=$(cargo run -q -p sel-validator-cli -- validate "$MISSION_FILE" \
           | grep -o "sel:v1.0:sha256:[a-f0-9]\+" || true)

    if [ -z "$HASH" ]; then
        echo "❌ Failed to extract hash at iteration $i"
        echo "   (CLI output did not contain sel:v1.0:sha256:...)"
        exit 1
    fi

    if [ -z "$FIRST_HASH" ]; then
        FIRST_HASH="$HASH"
        echo "  Iteration 1: $FIRST_HASH"
    elif [ "$HASH" != "$FIRST_HASH" ]; then
        echo "❌ Hash mismatch at iteration $i"
        echo "   Expected: $FIRST_HASH"
        echo "   Got:      $HASH"
        exit 1
    fi

    if [ $(( i % 10 )) -eq 0 ]; then
        echo "  Iteration $i/100: ✓"
    fi
done

echo ""
echo "✅ All 100 iterations produced identical hash: $FIRST_HASH"
echo ""
echo "=================================="
echo "✅ Core 1.0 Testing Marathon COMPLETE"
echo "=================================="
