#!/bin/bash

echo "⚡ SEL Engine - بدء فوري ⚡"
echo "=========================="

# البناء
echo -e "\n🔨 جاري البناء..."
cargo build

if [ $? -ne 0 ]; then
    echo "❌ فشل البناء. تحقق من Rust/cargo."
    exit 1
fi

echo "✅ تم البناء بنجاح!"

# اختبار المثال البسيط
echo -e "\n🧪 جاري اختبار المثال البسيط..."
echo "--------------------------------"

# إنشاء مهمة اختبار فورية
cat > quick_test.json << 'TEST_EOF'
{
  "id": "quick_test",
  "version": "1.0.0",
  "metadata": {"quick": true},
  "execution": {
    "actions": [{
      "id": 1,
      "type": "command",
      "command": "echo",
      "args": ["✅ SEL Engine يعمل!"],
      "working_directory": "/workspace/quick_test"
    }]
  }
}
TEST_EOF

# التحقق
echo -e "\n🔍 التحقق من المهمة..."
./target/debug/sel-engine validate --mission quick_test.json

# التنفيذ
echo -e "\n⚡ تنفيذ المهمة..."
./target/debug/sel-engine execute --mission quick_test.json

# عرض النتائج
echo -e "\n📊 النتائج:"
if [ -f "facts.jsonl" ]; then
    echo "✅ تم إنشاء facts.jsonl"
    echo "--- محتويات facts.jsonl ---"
    cat facts.jsonl
    echo "---------------------------"
else
    echo "❌ لم يتم إنشاء facts.jsonl"
fi

# تنظيف
rm -f quick_test.json facts.jsonl

echo -e "\n🎉 SEL Engine جاهز للاستخدام!"
echo -e "\nاستخدم:"
echo "  ./target/debug/sel-engine --help"
echo "  ./target/debug/sel-engine execute --mission mission.json"
