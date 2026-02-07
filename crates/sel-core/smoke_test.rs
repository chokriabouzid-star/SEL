fn main() {
    println!("✅ SEL Core compiled successfully!");
    println!("📊 Library size: {} KB", 
        std::fs::metadata("target/release/libsel_core.rlib")
            .map(|m| m.len() / 1024)
            .unwrap_or(0)
    );
}
