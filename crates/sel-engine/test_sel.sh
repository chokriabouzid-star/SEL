#!/bin/bash

echo "🧪 اختبار SEL Engine الشامل"
echo "==========================="

echo ""
echo "1. 🔍 التحقق من المهمة:"
./target/debug/sel-engine validate --mission examples/simple_mission.json
VALIDATE_RESULT=$?

echo ""
echo "2. 🚀 التنفيذ بدون تحقق:"
./target/debug/sel-engine execute --mission examples/simple_mission.json --no-validate
EXECUTE_NO_VALIDATE_RESULT=$?

echo ""
echo "3. ✅ التنفيذ مع تحقق:"
./target/debug/sel-engine execute --mission examples/simple_mission.json
EXECUTE_WITH_VALIDATE_RESULT=$?

echo ""
echo "4. 🔎 فحص المهمة:"
./target/debug/sel-engine inspect --mission examples/simple_mission.json

echo ""
echo "5. 📊 الحقائق المولدة:"
FACTS_FILES=$(find . -name "facts*.jsonl" -type f)
if [ -n "$FACTS_FILES" ]; then
    for file in $FACTS_FILES; do
        echo "📄 ملف: $file"
        echo "📈 عدد الأسطر: $(wc -l < "$file")"
        echo "📦 الحجم: $(wc -c < "$file") bytes"
        echo "--- أول 5 أسطر ---"
        head -5 "$file"
        echo "-------------------"
    done
else
    echo "❌ لم يتم إنشاء أي ملف حقائق"
fi

echo ""
echo "📋 ملخص النتائج:"
echo "  التحقق: $([ $VALIDATE_RESULT -eq 0 ] && echo "✅ نجح" || echo "❌ فشل")"
echo "  التنفيذ بدون تحقق: $([ $EXECUTE_NO_VALIDATE_RESULT -eq 0 ] && echo "✅ نجح" || echo "❌ فشل")"
echo "  التنفيذ مع تحقق: $([ $EXECUTE_WITH_VALIDATE_RESULT -eq 0 ] && echo "✅ نجح" || echo "❌ فشل")"

if [ $VALIDATE_RESULT -eq 0 ] && [ $EXECUTE_NO_VALIDATE_RESULT -eq 0 ]; then
    echo ""
    echo "🎉 SEL Engine يعمل بنجاح!"
    echo "🚀 جاهز للتكامل مع AI!"
else
    echo ""
    echo "⚠️  هناك بعض المشاكل تحتاج للإصلاح."
fi
