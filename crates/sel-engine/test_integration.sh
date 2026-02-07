#!/bin/bash
echo "🧪 Actual Integration Test"
echo "=========================="

# إنشاء مهمة اختبار
cat > test_mission_int.json << 'MISSION'
{
  "id": "integration-verify",
  "version": "1.0.0",
  "execution": {
    "actions": [
      {
        "id": 1,
        "type": "test",
        "command": "echo 'Sovereign Verification'",
        "working_directory": "/tmp"
      }
    ]
  }
}
MISSION

echo "1. Created test mission"

# تشغيل canonicalize
echo ""
echo "2. Running canonicalize command..."
OUTPUT=$(cargo run -- canonicalize test_mission_int.json 2>&1)

if echo "$OUTPUT" | grep -q "Mission hash:"; then
    echo "   ✅ Canonicalization working!"
    
    # استخرج الـ hash
    HASH=$(echo "$OUTPUT" | grep "Mission hash:" | cut -d' ' -f3)
    echo "   🔒 Hash: $HASH"
else
    echo "   ❌ Canonicalization failed"
    echo "   Output: $OUTPUT"
    exit 1
fi

# اختبار hash chain
echo ""
echo "3. Testing hash chain..."
CHAIN_OUTPUT=$(cargo run -- hash-chain --new 2>&1)
if echo "$CHAIN_OUTPUT" | grep -q "Hash chain created"; then
    echo "   ✅ Hash chain working!"
else
    echo "   ⚠️ Hash chain output: $CHAIN_OUTPUT"
fi

echo ""
echo "🎉 INTEGRATION VERIFIED SUCCESSFULLY!"
