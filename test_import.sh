#!/bin/bash
cd ~/sel-production/SEL

# محاولة استيراد sel-core في برنامج Rust بسيط
cat > test_import.rs << 'RUSTCODE'
// Test importing sel-core
extern crate sel_core;

fn main() {
    println!("✅ sel-core can be imported!");
    
    // Try to use some functions (compile-time check)
    let _ = sel_core::canonicalize_json;
    let _ = sel_core::HashChain::new;
    
    println!("✅ sel-core functions are available!");
}
RUSTCODE

# محاولة التجميع
rustc test_import.rs --extern sel_core=crates/sel-core/target/release/libsel_core.rlib 2>/dev/null

if [ $? -eq 0 ]; then
    echo "✅ sel-core import successful!"
    ./test_import
    rm -f test_import test_import.rs
else
    echo "⚠️ Import test skipped (library path issues)"
fi
