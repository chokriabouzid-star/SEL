#!/bin/bash
set -e

echo "🧪 اختبار نهائي لليوم الرابع"
echo "=============================="
echo ""

cd ~/sel-production/SEL

echo "1. 🔨 بناء المشروع..."
cargo build --release

echo ""
echo "2. 🧪 تشغيل جميع الاختبارات..."
cargo test --release

echo ""
echo "3. 🔒 اختبارات الأمان:"
echo "   a) Path traversal..."
cat > /tmp/pt1.json << 'JSON'
{
  "name": "pt1",
  "actions": [
    {"type": "command", "command": "cat", "args": ["../../etc/passwd"]}
  ]
}
JSON
./target/release/sel-engine validate /tmp/pt1.json 2>/dev/null || echo "     ✅ تم رفضه"

echo "   b) Forbidden command..."
cat > /tmp/pt2.json << 'JSON'
{
  "name": "pt2",
  "actions": [
    {"type": "command", "command": "rm", "args": ["-rf", "/"]}
  ]
}
JSON
./target/release/sel-engine validate /tmp/pt2.json 2>/dev/null || echo "     ✅ تم رفضه"

echo "   c) Valid mission..."
cat > /tmp/pt3.json << 'JSON'
{
  "name": "pt3",
  "actions": [
    {"type": "command", "command": "echo", "args": ["test"]}
  ]
}
JSON
./target/release/sel-engine validate /tmp/pt3.json && echo "     ✅ تم قبوله"

echo ""
echo "4. 📊 حالة المشروع:"
echo "   - sel-core: $(cargo test -p sel-core -- --quiet 2>/dev/null && echo '✅' || echo '❌')"
echo "   - sel-validator: $(cargo test -p sel-validator -- --quiet 2>/dev/null && echo '✅' || echo '❌')"
echo "   - sel-engine CLI: $(./target/release/sel-engine --version >/dev/null 2>&1 && echo '✅' || echo '❌')"

echo ""
echo "🎉 اختبار اليوم الرابع مكتمل!"
