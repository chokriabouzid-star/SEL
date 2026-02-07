# 🛡️ SEL Engine - Sovereign Execution Layer

محرك تنفيذ حتمي بحت، لا ذكاء، لا تحليل، فقط تنفيذ وتسجيل حقائق.

## 🎯 المبادئ الدستورية

1. **العزل المعرفي**: SEL لا يعرف مصدر المهمة
2. **النقاء الوصفي**: يسجل حقائق فقط، لا تحليل
3. **الحتمية المطلقة**: نفس المدخلات → نفس المخرجات

## 📦 التثبيت السريع

```bash
# استنساخ المشروع
git clone <repo-url>
cd sel-engine

# البناء
cargo build --release

# النسخة جاهزة
cp target/release/sel-engine /usr/local/bin/
🚀 الاستخدام الفوري
1. إنشاء مهمة تجريبية:
bash
cat > test.json << 'EOF'
{
  "id": "test",
  "version": "1.0.0",
  "metadata": {"test": true},
  "execution": {
    "actions": [{
      "id": 1,
      "type": "command",
      "command": "echo",
      "args": ["SEL يعمل!"],
      "working_directory": "/workspace/test"
    }]
  }
}
