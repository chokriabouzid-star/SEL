// Test importing sel-core
extern crate sel_core;

fn main() {
    println!("✅ sel-core can be imported!");
    
    // Try to use some functions (compile-time check)
    let _ = sel_core::canonicalize_json;
    let _ = sel_core::HashChain::new;
    
    println!("✅ sel-core functions are available!");
}
