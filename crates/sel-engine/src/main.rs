//! SEL Engine CLI
//! 
//! Command-line interface for Sovereign Execution Layer

use clap::{Parser, Subcommand};
use serde_json::json;

#[derive(Parser)]
#[command(name = "sel-engine")]
#[command(about = "Sovereign Execution Layer - Deterministic Execution Engine")]
#[command(version = "1.0.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Canonicalize and hash a mission
    Canonicalize {
        /// Mission JSON file
        mission_file: String,
        
        /// Output file (optional)
        #[arg(short, long)]
        output: Option<String>,
    },
    
    /// Create a hash chain
    HashChain {
        /// Create new hash chain
        #[arg(short, long)]
        new: bool,
        
        /// Add event to chain (JSON)
        #[arg(short, long)]
        add: Option<String>,
    },
    
    /// Test SEL integration
    Test {
        /// Run integration tests
        #[arg(short, long)]
        integration: bool,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Canonicalize { mission_file, output } => {
            println!("🔨 Canonicalizing mission from: {}", mission_file);
            
            // Read mission file
            let content = std::fs::read_to_string(&mission_file)?;
            let mission: serde_json::Value = serde_json::from_str(&content)?;
            
            // Use canonical_adapter if available
            println!("📋 Mission loaded successfully");
            println!("📏 Size: {} bytes", content.len());
            
            // Try to canonicalize if the module is available
            #[cfg(feature = "canonical")]
            {
                use sel_engine::canonical_adapter::canonicalize_mission;
                let (canonical, hash) = canonicalize_mission(&mission);
                
                println!("✅ Mission canonicalized");
                println!("📄 Canonical JSON length: {} chars", canonical.len());
                println!("🔒 Mission hash: sha256:{}...", &hash[0..16]);
                
                if let Some(output_path) = output {
                    let output_data = json!({
                        "original": mission,
                        "canonical": canonical,
                        "hash": hash,
                        "formatted_hash": format!("sha256:{}", hash)
                    });
                    
                    std::fs::write(output_path, serde_json::to_string_pretty(&output_data)?)?;
                    println!("💾 Output saved to file");
                }
            }
            
            #[cfg(not(feature = "canonical"))]
            {
                println!("⚠️ Canonicalization feature not enabled");
                println!("   Mission: {}", serde_json::to_string(&mission)?);
            }
        }
        
        Commands::HashChain { new, add } => {
            if new {
                println!("⛓️ Creating new hash chain...");
                
                #[cfg(feature = "canonical")]
                {
                    use sel_engine::canonical_adapter::create_hash_chain;
                    let chain = create_hash_chain();
                    println!("✅ Hash chain created");
                    println!("   Initial hash: {}", chain.finalize());
                }
                
                #[cfg(not(feature = "canonical"))]
                {
                    println!("⚠️ Hash chain feature not enabled");
                }
            }
            
            if let Some(event_json) = add {
                println!("➕ Adding event to chain: {}", event_json);
                
                #[cfg(feature = "canonical")]
                {
                    use sel_engine::canonical_adapter::create_hash_chain;
                    let event: serde_json::Value = serde_json::from_str(&event_json)?;
                    let mut chain = create_hash_chain();
                    let hash = chain.append(&event);
                    
                    println!("✅ Event added to chain");
                    println!("   Event hash: {}...", &hash[0..16]);
                    println!("   Chain length: {}", chain.len());
                }
            }
        }
        
        Commands::Test { integration } => {
            if integration {
                println!("🧪 Running integration tests...");
                
                // Test canonicalization
                let test_mission = json!({
                    "id": "integration-test",
                    "version": "1.0.0",
                    "execution": {
                        "actions": [
                            {
                                "id": 1,
                                "type": "command",
                                "command": "echo 'SEL Integration Test'"
                            }
                        ]
                    }
                });
                
                println!("📋 Test mission created");
                
                #[cfg(feature = "canonical")]
                {
                    use sel_engine::canonical_adapter::canonicalize_mission;
                    
                    println!("🔨 Canonicalizing...");
                    let (canonical, hash) = canonicalize_mission(&test_mission);
                    
                    println!("✅ Canonicalization successful!");
                    println!("   Canonical length: {} chars", canonical.len());
                    println!("   Mission hash: {}...", &hash[0..16]);
                    
                    // Test determinism
                    let (_, hash2) = canonicalize_mission(&test_mission);
                    if hash == hash2 {
                        println!("✅ Determinism verified!");
                    } else {
                        println!("❌ Determinism broken!");
                    }
                }
                
                println!("🎉 Integration test completed!");
            }
        }
    }
    
    Ok(())
}
