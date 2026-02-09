#!/bin/bash
set -e

echo "🛡️ Day 4: The Shield - Starting Implementation"
echo "================================================"
echo ""

cd ~/sel-production/SEL

# Clean build
echo "🧹 Cleaning previous build..."
cargo clean

# Build all crates
echo "🔨 Building sel-validator..."
cd crates/sel-validator
cargo build --release

echo "🔨 Building sel-engine..."
cd ../sel-engine
cargo build --release

cd ../..

# Run tests
echo ""
echo "🧪 Running sel-validator tests..."
cd crates/sel-validator
cargo test --release

echo ""
echo "🧪 Running sel-engine tests..."
cd ../sel-engine
cargo test --release

cd ../..

# Run integration tests
echo ""
echo "🧪 Running integration tests..."
cargo test --release --test day4_integration

# Verify builds
echo ""
echo "📊 Build Verification:"
ls -lh crates/sel-validator/target/release/libsel_validator.rlib 2>/dev/null || echo "  sel-validator: ⚠️ Not found"
ls -lh crates/sel-engine/target/release/sel-engine 2>/dev/null || echo "  sel-engine: ⚠️ Not found"

echo ""
echo "✅ Day 4 Implementation Complete!"
echo ""
echo "🎯 Next Steps:"
echo "   1. Review test results above"
echo "   2. Run: ./test_day4_scenarios.sh"
echo "   3. Commit changes to Git"
