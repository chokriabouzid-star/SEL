#!/bin/bash
echo "🔍 اختبار مسارات SEL CLI..."
echo ""

# اختبار 1: البحث عن sel-engine
echo "1. البحث عن ملف sel-engine التنفيذي:"
find . -name "sel-engine" -type f -executable 2>/dev/null | head -5

echo ""
echo "2. اختبار مع المسار الصحيح:"
if [ -f "./target/release/sel-engine" ]; then
    echo "   ✅ وجد: ./target/release/sel-engine"
    ./target/release/sel-engine --version
else
    echo "   ❌ لم يوجد في ./target/release/sel-engine"
fi

echo ""
echo "3. البحث في crates:"
if [ -f "./crates/sel-engine/target/release/sel-engine" ]; then
    echo "   ✅ وجد: ./crates/sel-engine/target/release/sel-engine"
else
    echo "   ❌ لم يوجد في crates"
fi
