#!/bin/bash
set -e

echo "🔍 SEL Determinism Verification"
echo "================================"

cd ~/sel-production/SEL

# 1. Check forbidden patterns
echo "1️⃣ Checking forbidden patterns..."
if command -v rg &> /dev/null; then
    if rg "HashMap" --type rust crates/sel-{common,validator}/src | grep -v test | grep -v "//"; then
        echo "❌ Found HashMap in critical path"
        exit 1
    fi
    
    if rg "Instant|SystemTime" --type rust crates/sel-{common,validator}/src | grep -v test; then
        echo "❌ Found wall time in critical path"
        exit 1
    fi
    echo "✅ No forbidden patterns"
else
    echo "⚠️  ripgrep not installed, skipping pattern check"
fi

# 2. Build
echo ""
echo "2️⃣ Building..."
cargo build --release --workspace

# 3. Run tests
echo ""
echo "3️⃣ Running tests..."
cargo test --workspace

echo ""
echo "✅ Determinism verification complete"
