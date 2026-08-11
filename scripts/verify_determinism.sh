#!/bin/bash
# SEL Determinism Verification
# Usage: bash scripts/verify_determinism.sh (from any directory)
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

echo "🔍 SEL Determinism Verification"
echo "================================"
echo "   Repo: $REPO_ROOT"
echo ""

cd "$REPO_ROOT"

# ── 1. Forbidden patterns ─────────────────────────────────────────────────
echo "1️⃣  Checking forbidden patterns..."
if command -v rg &> /dev/null; then
    if rg "HashMap" --type rust crates/sel-common/src crates/sel-validator/src \
       | grep -v test | grep -v "//"; then
        echo "❌ Found HashMap in critical path"
        exit 1
    fi

    if rg "Instant|SystemTime" --type rust \
       crates/sel-common/src crates/sel-validator/src \
       | grep -v test; then
        echo "❌ Found wall time in critical path"
        exit 1
    fi
    echo "✅ No forbidden patterns"
else
    echo "⚠️  ripgrep not installed, skipping pattern check"
fi

# ── 2. Build ──────────────────────────────────────────────────────────────
echo ""
echo "2️⃣  Building..."
cargo build --release --workspace

# ── 3. Tests ──────────────────────────────────────────────────────────────
echo ""
echo "3️⃣  Running tests..."
cargo test --workspace

echo ""
echo "✅ Determinism verification complete"
