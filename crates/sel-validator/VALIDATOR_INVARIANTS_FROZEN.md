# VALIDATOR INVARIANTS v0.1 - FROZEN
## الوثيقة السيادية المجمدة

### التاريخ: 2026-02-04
### الحالة: FROZEN - لا تغيير حتى v1.0

## المادة 1: المبادئ غير القابلة للكسر

### 1.1 العمى الدلالي
Validator أعمى عن:
- معنى المهمة
- نية كاتبها  
- احتمالية نجاحها
- فائدة نتيجتها

### 1.2 المطابقة الحرفية فقط
مسموح فقط:
- ExactString (تطابق نصي تام)
- Presence (وجود/عدم وجود)
- RangeNumeric (نطاق عددي)
- EnumValue (قيمة من مجموعة محددة)

ممنوع:
- SemanticSimilarity
- PatternRecognition  
- IntentInference

### 1.3 الصمت العلاجي
عند الرفض، Validator يقدم فقط:
- اسم القاعدة المنتهكة
- موقع الانتهاك (JSON path)
- الوصف الحرفي للمشكلة

لا يقدم:
- اقتراح إصلاح
- تحليل للسبب
- تقدير للخطورة
- نصيحة للمستخدم

## المادة 2: قواعد v0.1 المجمدة

### 2.1 قواعد الوجود (8 قواعد)
1. MissionHasId
2. MissionHasVersion  
3. MissionHasExecution
4. ExecutionHasActions
5. ActionsNonEmpty
6. EachActionHasId
7. EachActionHasCommand
8. EachActionHasVerification

### 2.2 قواعد التطابق (6 قواعد)
1. IdMatchesRegex: "^[a-z0-9-]+$"
2. VersionMatchesSemver
3. ActionTypeIsCommand (القيمة الوحيدة المسموحة)
4. WorkingDirectoryContainsWorkspaceVar (فقط ${workspace})
5. TimeoutBetween1And3600
6. CommandLengthUnder1000Chars

### 2.3 قواعد السلامة (4 قواعد)
1. NoForbiddenCommands: ["rm -rf", "sudo", "chmod 777", "curl", "wget", "ssh", "nc"]
2. NoNetworkFlags: ["--network", "-n", "--fetch"]
3. NoSystemPaths: ["/bin", "/usr", "/etc", "/home"]
4. NoShellControlChars: ["&&", "||", ";", "|"]

## المادة 3: التجميد والعقوبات

### 3.1 حالة التجميد
- الوثيقة مجمدة حتى v1.0
- أي تغيير في المبادئ = كسر دستوري
- أي إضافة "ذكاء" = كسر دستوري

### 3.2 عقوبات الكسر
- ValidatorReadsMetadata: إعادة الكتابة من الصفر
- ValidatorSuggestsFixes: إزالة الميزة وإضافة اختبار
- أي كسر = عودة للنقطة صفر

## التوقيع السيادي
أنا الموقع أدناه أقر بأن:
1. فهمت مبادئ Validator غير القابلة للكسر
2. أوافق على تجميد هذا التصميم حتى v1.0
3. أي كسر للدستور سيتطلب إعادة كتابة كاملة

التوقيع: ________________
التاريخ: 2026-02-04
