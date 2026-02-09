# 🎉 SEL - جميع الأيام 1-4 تعمل بنجاح!

## 🚀 **الأوامر المتاحة:**

### **1. Canonicalize JSON (اليوم 2)**
```bash
./crates/sel-engine/target/release/sel-engine canonicalize mission.json
المخرجات: JSON متعارف عليه حسب RFC 8785

2. Hash Chain (اليوم 2)
bash
./crates/sel-engine/target/release/sel-engine hash-chain file1.json file2.json file3.json
المخرجات: سلسلة تجزئة مرتبطة

3. Validate Mission (اليوم 4)
bash
# تحقق أساسي
./crates/sel-engine/target/release/sel-engine validate mission.json

# تحقق مفصل
./crates/sel-engine/target/release/sel-engine validate mission.json -v
4. Run Tests
bash
# جميع الاختبارات
./crates/sel-engine/target/release/sel-engine test all

# اختبار محدد
./crates/sel-engine/target/release/sel-engine test canonical
./crates/sel-engine/target/release/sel-engine test hash  
./crates/sel-engine/target/release/sel-engine test validate
📋 أمثلة عملية:
✅ مهمة صالحة:
json
{
  "name": "backup",
  "actions": [
    {"type": "command", "command": "echo", "args": ["Starting backup"]},
    {"type": "command", "command": "ls", "args": ["-la", "/home"]}
  ]
}
النتيجة: ✅ VALIDATION PASSED

❌ مهمة خطيرة:
json
{
  "name": "dangerous",
  "actions": [
    {"type": "command", "command": "rm", "args": ["-rf", "/"]}
  ]
}
النتيجة: ❌ VALIDATION FAILED - Forbidden command: rm

🏗️ الهيكل الفني:
sel-core (اليوم 2)
canonicalize_json() - تطبيق RFC 8785

HashChain - سلاسل تجزئة حتمية

sel-validator (اليوم 4)
Validator::new() - محقق سيادي

فحص الأوامر المحظورة: rm, dd, shutdown, إلخ

توليد برهان HMAC-SHA256
