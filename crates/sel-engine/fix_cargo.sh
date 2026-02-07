#!/bin/bash
cd ~/sel-production/SEL/crates/sel-engine

# احفظ نسخة احتياطية
cp Cargo.toml Cargo.toml.backup

# أضف sel-core فقط (إذا لم يكن موجوداً)
if ! grep -q "sel-core" Cargo.toml; then
    cat >> Cargo.toml << 'ADD'

[dependencies.sel-core]
path = "../sel-core"
version = "0.1.0"
ADD
    echo "✅ Added sel-core dependency"
else
    echo "⚠️ sel-core already exists in dependencies"
fi

# تحقق من sha2
if ! grep -q "sha2" Cargo.toml; then
    cat >> Cargo.toml << 'ADD'

[dependencies.sha2]
version = "0.10"
ADD
    echo "✅ Added sha2 dependency"
else
    echo "⚠️ sha2 already exists in dependencies"
fi
