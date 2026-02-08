#!/bin/bash
set -e

echo "=== Day 3: Determinism Test ==="
echo ""

# Run 1
echo "🔄 Run 1: Canonicalizing..."
./crates/sel-engine/target/release/sel-engine canonicalize \
  test_day3.json > canonical_run1.txt 2>&1

# Run 2
echo "🔄 Run 2: Canonicalizing..."
./crates/sel-engine/target/release/sel-engine canonicalize \
  test_day3.json > canonical_run2.txt 2>&1

# Extract full hash (improved pattern)
HASH1=$(grep "🔒 Mission hash:" canonical_run1.txt | sed 's/.*sha256:/sha256:/' | cut -d' ' -f1)
HASH2=$(grep "🔒 Mission hash:" canonical_run2.txt | sed 's/.*sha256:/sha256:/' | cut -d' ' -f1)

echo ""
echo "📊 Results:"
echo "   Run 1 hash: $HASH1"
echo "   Run 2 hash: $HASH2"
echo ""

if [ "$HASH1" = "$HASH2" ] && [ ! -z "$HASH1" ]; then
    echo "✅ DETERMINISM VERIFIED!"
    echo "   Same mission → Same hash"
    echo ""
    echo "🎯 Day 3 SUCCESS CRITERIA MET:"
    echo "   1. Canonicalization works ✅"
    echo "   2. Hash is deterministic ✅"
    echo "   3. Sovereign DNA operational ✅"
    echo ""
    echo "📄 Hash value:"
    echo "   $HASH1"
    exit 0
else
    echo "❌ DETERMINISM FAILED or extraction error"
    echo ""
    echo "Debug info:"
    echo "--- Run 1 output ---"
    cat canonical_run1.txt
    echo ""
    echo "--- Run 2 output ---"
    cat canonical_run2.txt
    exit 1
fi
