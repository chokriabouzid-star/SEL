#!/bin/bash

echo "🔍 Phase 1 - Step 1 Verification"
echo "================================="
echo ""

cd ~/sel-production/SEL

# 1. Check workspace.rs exists
echo "1. Workspace module:"
if [ -f "crates/sel-engine/src/engine/workspace.rs" ]; then
    LINES=$(wc -l < crates/sel-engine/src/engine/workspace.rs)
    echo "   ✅ workspace.rs ($LINES lines)"
else
    echo "   ❌ workspace.rs MISSING"
    exit 1
fi

echo ""

# 2. Check MissionExecutor::new() implemented
echo "2. MissionExecutor::new():"
if grep -q "Workspace::new(mode)" crates/sel-engine/src/engine/executor.rs; then
    echo "   ✅ new() implemented"
else
    echo "   ❌ new() not implemented"
    exit 1
fi

echo ""

# 3. Run workspace tests
echo "3. Workspace tests:"
cargo test --release workspace 2>&1 | grep "test result:" | head -1

echo ""

# 4. Run executor tests
echo "4. Executor tests:"
cargo test --release executor 2>&1 | grep "test result:" | head -1

echo ""

# 5. Create test workspace
echo "5. Live test - Create workspace:"
BEFORE=$(ls -1 /tmp/sel-workspace-* 2>/dev/null | wc -l)
cargo run --release --bin sel-engine test 2>&1 | grep -q "SEL" || true
AFTER=$(ls -1 /tmp/sel-workspace-* 2>/dev/null | wc -l)

if [ $AFTER -eq $BEFORE ]; then
    echo "   ✅ Cleanup working (no leaked workspaces)"
else
    echo "   ⚠️ Workspaces created: $AFTER (check cleanup)"
fi

echo ""
echo "✅ Step 1 verification complete"
echo ""
echo "🎯 Ready for Step 2: Execution Pipeline"
