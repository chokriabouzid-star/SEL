use sel_validator::HmacAuthority;
use sel_validator::SignatureAuthority;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let auth = HmacAuthority::test_key();
    let corpus_dir = "fuzz/corpus/signature";
    
    println!("🔍 CHECKING DETERMINISM FOR {} FILES", corpus_dir);
    println!("========================================");
    
    let mut total = 0;
    let mut failed = 0;
    
    for entry in fs::read_dir(corpus_dir)? {
        let path = entry?.path();
        let data = fs::read(&path)?;
        
        // Sign twice
        let sig1 = auth.sign(&data)?;
        let sig2 = auth.sign(&data)?;
        
        if sig1 == sig2 {
            println!("✅ {:32} → OK", path.file_name().unwrap().to_string_lossy());
        } else {
            println!("❌ {:32} → DETERMINISM BROKEN!", path.file_name().unwrap().to_string_lossy());
            failed += 1;
        }
        total += 1;
    }
    
    println!("\n📊 SUMMARY: {}/{} passed", total - failed, total);
    if failed == 0 {
        println!("✅ ALL INPUTS ARE DETERMINISTIC");
    } else {
        println!("❌ {} INPUTS BROKE DETERMINISM", failed);
    }
    
    Ok(())
}
