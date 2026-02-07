# ⚡ الإعداد الفوري لـ SEL Engine

## الخطوة 1: التحقق من المتطلبات
```bash
# تأكد من تثبيت Rust
rustc --version
cargo --version
الخطوة 2: البناء
bash
./build_and_test.sh
الخطوة 3: اختبار فوري
bash
# 1. اختبر المثال البسيط
sel-engine validate --mission examples/simple_mission.json
sel-engine execute --mission examples/simple_mission.json

# 2. شاهد الحقائق
cat facts.jsonl | python3 -m json.tool

# 3. جرب تكامل AI
python3 examples/ai_integration.py
الخطوة 4: استخدام مع AI حقيقي
AI يولد mission.json

sel-engine execute --mission mission.json

AI يقرأ facts.jsonl

AI يصحح ويولد مهمة جديدة

🔧 استكشاف الأخطاء
bash
# إذا كان هناك خطأ في التحقق:
# تأكد من وجود SEL Validator في ../sel-validator
# أو عيّن المسار في Cargo.toml

# إذا فشل التنفيذ:
# تأكد من صلاحيات التنفيذ
# تحقق من وجود الأوامر المطلوبة
📞 المساعدة
اقرأ README.md

جرب الأمثلة في examples/

راجع الكود في src/
