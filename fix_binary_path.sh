#!/bin/bash

# Find the actual binary
BINARY=$(find ~/sel-production/SEL -name "sel-engine" -type f | grep release | head -1)

if [ -z "$BINARY" ]; then
    echo "❌ Binary not found. Building..."
    cd ~/sel-production/SEL
    cargo build --release --bin sel-engine
    BINARY=$(find ~/sel-production/SEL -name "sel-engine" -type f | grep release | head -1)
fi

echo "✅ Binary found: $BINARY"
echo ""
echo "Testing CLI:"
$BINARY --help | head -20
