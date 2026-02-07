use sel_engine::canonical_adapter::canonicalize_mission;
use serde_json::json;

fn main() {
    println!("🧪 Testing SEL Core integration...");
    
    let mission = json!({
        "name": "integration-test",
        "version": "1.0.0",
        "metadata": {"author": "SEL"},
        "execution": {
            "actions": [
                {
                    "type": "command",
                    "command": "echo 'SEL + SEL Core = Sovereign'"
                }
            ]
        }
    });
    
    match canonicalize_mission(&mission) {
        Ok((canonical, hash)) => {
            println!("✅ Integration successful!");
            println!("   Canonical length: {} chars", canonical.len());
            println!("   Mission hash: {}...", &hash[0..16]);
            println!("   Hash length: {} chars", hash.len());
        }
        Err(e) => {
            println!("❌ Integration failed: {}", e);
        }
    }
}
