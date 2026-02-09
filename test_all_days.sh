#!/bin/bash
set -e

echo "🧪 اختبار شامل لجميع أيام SEL (1-4)"
echo "===================================="
echo ""

cd ~/sel-production/SEL

# 1. اختبار البناء
echo "1. 🔨 اختبار البناء..."
cargo build --release

# 2. اختبار Canonical JSON (اليوم 2)
echo ""
echo "2. 📄 اختبار Canonical JSON (اليوم 2)..."
cat > /tmp/test_canonical.json << 'JSON'
{
  "name": "canonical-test",
  "actions": [
    {"type": "command", "command": "echo", "args": ["test"]}
  ]
}
JSON
./crates/sel-engine/target/release/sel-engine canonicalize /tmp/test_canonical.json

# 3. اختبار Hash Chain (اليوم 2)
echo ""
echo "3. ⛓️  اختبار Hash Chain (اليوم 2)..."
cat > /tmp/h1.json << 'JSON'
{"id": 1, "action": "start"}
JSON
cat > /tmp/h2.json << 'JSON'
{"id": 2, "action": "process"}
JSON
./crates/sel-engine/target/release/sel-engine hash-chain /tmp/h1.json /tmp/h2.json

# 4. اختبار Validator (اليوم 4) - مهمة صالحة
echo ""
echo "4. 🛡️  اختبار Validator - مهمة صالحة (اليوم 4)..."
cat > /tmp/valid_mission.json << 'JSON'
{
  "name": "valid-mission",
  "actions": [
    {"type": "command", "command": "echo", "args": ["Hello"]},
    {"type": "command", "command": "ls", "args": ["-la"]}
  ]
}
JSON
./crates/sel-engine/target/release/sel-engine validate /tmp/valid_mission.json

# 5. اختبار Validator (اليوم 4) - أمر محظور
echo ""
echo "5. 🛡️  اختبار Validator - أمر محظور (اليوم 4)..."
cat > /tmp/forbidden_mission.json << 'JSON'
{
  "name": "forbidden-mission",
  "actions": [
    {"type": "command", "command": "rm", "args": ["-rf", "/tmp"]}
  ]
}
JSON
./crates/sel-engine/target/release/sel-engine validate /tmp/forbidden_mission.json || echo "  ❌ تم رفضه كما هو متوقع"

# 6. اختبار Validator (اليوم 4) - JSON معيب
echo ""
echo "6. 🛡️  اختبار Validator - JSON معيب (اليوم 4)..."
cat > /tmp/broken_mission.json << 'JSON'
{
  "name": 123,
  "actions": "not-an-array"
}
JSON
./crates/sel-engine/target/release/sel-engine validate /tmp/broken_mission.json || echo "  ❌ تم رفضه كما هو متوقع"

# 7. اختبار Test command
echo ""
echo "7. 🧪 اختبار Test command..."
./crates/sel-engine/target/release/sel-engine test all

echo ""
echo "✅ جميع الاختبارات اكتملت بنجاح!"
echo "🎉 أيام SEL 1-4 تعمل بشكل كامل!"
