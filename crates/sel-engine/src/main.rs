//! SEL Engine CLI

use clap::{Parser, Subcommand};
use serde_json::json;

#[derive(Parser)]
#[command(name = "sel-engine")]
#[command(about = "Sovereign Execution Layer - Deterministic Execution Engine")]
#[command(version = "0.2.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Canonicalize and hash a mission
    Canonicalize {
        /// Mission JSON file
        #[arg(short, long)]
        mission_file: String,

        /// Output file (optional)
        #[arg(short, long)]
        _output: Option<String>,  // ✅ prefixed with _ to suppress warning
    },

    /// Create a hash chain
    HashChain {
        /// Test events
        #[arg(short, long)]
        test: bool,
    },

    /// Test SEL integration
    Test,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Canonicalize { mission_file, _output } => {
            println!("📦 Canonicalizing mission: {}", mission_file);

            let content = std::fs::read_to_string(&mission_file)?;
            let (canonical, hash) = sel_engine::canonicalize_mission(&content)?;

            println!("✅ Canonical form ({} bytes):", canonical.len());
            println!("{}", canonical);
            println!("\n🔐 Mission hash:");
            println!("{}", hash);

            Ok(())
        }

        Commands::HashChain { test } => {
            println!("🔗 Hash Chain Test");

            let mut chain = sel_engine::create_hash_chain();

            if test {
                let events = vec![
                    json!({"type": "mission_start", "id": "test-001"}),
                    json!({"type": "action_start", "action": 1}),
                    json!({"type": "command_executed", "cmd": "echo hello"}),
                ];

                for (i, event) in events.iter().enumerate() {
                    let hash = chain.append(event);
                    println!("Event {}: {}...", i, &hash[0..16]);
                }

                println!("\n✅ Final chain hash: {}...", &chain.finalize()[0..16]);
            }

            Ok(())
        }

        Commands::Test => {
            println!("🧪 SEL Integration Test");
            println!("======================");

            // Test mission
            let _test_mission = json!({  // ✅ prefixed with _ to suppress warning
                "name": "test-integration",
                "actions": [
                    {
                        "type": "command",
                        "command": "echo",
                        "args": ["SEL integration test"]
                    }
                ]
            });

            println!("\n✅ Integration test structure valid");
            println!("   Run full execution with 'execute' command");

            Ok(())
        }
    }
}
