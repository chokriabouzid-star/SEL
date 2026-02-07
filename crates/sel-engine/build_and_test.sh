#!/bin/bash

echo "🔨 بناء SEL Engine..."
cargo build --release

if [ $? -eq 0 ]; then
    echo "✅ تم البناء بنجاح"
    
    echo ""
    echo "🧪 اختبار SEL Engine..."
    
    # اختبار بسيط
    ./target/release/sel-engine --help
    
    echo ""
    echo "📋 أوامر متاحة:"
    echo "  sel-engine execute --mission mission.json"
    echo "  sel-engine validate --mission mission.json"
    echo "  sel-engine inspect --mission mission.json"
    
    echo ""
    echo "🎯 جرب مثالاً:"
    echo "  sel-engine validate --mission examples/simple_mission.json"
    echo "  sel-engine execute --mission examples/simple_mission.json"
    
else
    echo "❌ فشل البناء"
    exit 1
fi
