# SEL Validator v0.1.0
## Sovereign Execution Layer - Mechanical Pattern Matcher

### المبادئ الدستورية (مجمد)
- ✅ Validator أعمى دلالياً (لا يفهم النية)
- ✅ Validator لا يقرأ metadata (معاملة كـ opaque blob)
- ✅ Validator لا يقترح إصلاحات (صمت علاجي)
- ✅ التحقق الميكانيكي فقط (18 قاعدة صارمة)

### الهيكل
sel-validator/
├── src/
│ ├── types/ # تعريفات الأنواع
│ ├── validator/ # محرك التحقق والقواعد
│ ├── lib.rs # واجهة المكتبة
│ └── main.rs # CLI
├── tests/ # اختبارات دستورية
├── examples/ # أمثلة مهمات
└── VALIDATOR_INVARIANTS_FROZEN.md # الوثيقة المجمدة

text

### الاستخدام
```bash
# تحقق من مهمة
cargo run -- validate mission.json

# تحقق مع تفاصيل
cargo run -- validate mission.json --verbose

# عرض جميع القواعد
cargo run -- rules

# تشغيل اختبار دستوري
cargo run -- test
القواعد المطبقة (18 قاعدة)
قواعد الوجود (8): التحقق من وجود الحقول الإجبارية

قواعد التطابق (6): التحقق من التنسيقات والأنماط

قواعد السلامة (4): منع الأوامر الخطرة

التثبيت
bash
# كأداة CLI
cargo install --path .

# كمكتبة Rust
sel-validator = { git = "https://github.com/your-repo/sel-validator" }
التطوير
bash
# تشغيل جميع الاختبارات
./tests/run_tests.sh

# اختبارات الدستورية فقط
cargo test --test constitutional_tests

# بناء الوثائق
cargo doc --open
الرخصة
MIT License - مشروع القسطاس السيادي
