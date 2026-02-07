#!/bin/bash
echo "🔬 SEL Validator Constitutional Tests"
echo "====================================="

# اختبار البناء
echo -n "Building validator... "
cargo build --quiet
if [ $? -eq 0 ]; then
    echo "✅"
else
    echo "❌"
    exit 1
fi

# اختبارات الوحدة
echo -n "Running unit tests... "
cargo test --quiet --lib
if [ $? -eq 0 ]; then
    echo "✅"
else
    echo "❌"
    exit 1
fi

# اختبارات الدستورية
echo -n "Running constitutional tests... "
cargo test --quiet --test constitutional_tests
if [ $? -eq 0 ]; then
    echo "✅"
else
    echo "❌"
    exit 1
fi

# اختبار CLI
echo -n "Testing CLI with valid mission... "
cargo run --quiet -- validate examples/valid_mission.json > /dev/null 2>&1
if [ $? -eq 0 ]; then
    echo "✅"
else
    echo "❌"
    exit 1
fi

echo -n "Testing CLI with invalid mission... "
cargo run --quiet -- validate examples/invalid_mission.json > /dev/null 2>&1
if [ $? -ne 0 ]; then
    echo "✅"
else
    echo "❌"
    exit 1
fi

echo ""
echo "🎉 All constitutional tests passed!"
echo "SEL Validator v0.1.0 is ready."
