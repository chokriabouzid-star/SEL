use clap::{Parser, Subcommand};
use sel_core::{canonicalize_json, HashChain};
use sel_validator::{Validator, ValidationResult};
use serde_json::{Value, json};
use std::fs;

#[derive(Parser)]
#[command(author, version, about = "Sovereign Execution Layer", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Canonicalize JSON (Day 2)
    Canonicalize {
        /// JSON file to canonicalize
        file: String,
    },
    
    /// Create hash chain (Day 2)
    HashChain {
        /// JSON files to add to chain
        files: Vec<String>,
    },
    
    /// Validate mission (Day 4)
    Validate {
        /// Mission JSON file
        mission_file: String,
        
        /// Show detailed validation report
        #[arg(short, long)]
        verbose: bool,
    },
    
    /// Test SEL components
    Test {
        /// Component to test (all, canonical, hash, validate)
        #[arg(default_value = "all")]
        component: String,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Canonicalize { file } => {
            println!("🔤 SEL Canonical JSON (Day 2)");
            println!("📄 File: {}", file);
            println!("{}", "=".repeat(50));
            
            let content = fs::read_to_string(&file)?;
            let json: Value = serde_json::from_str(&content)?;
            
            let canonical = canonicalize_json(&json);
            println!("✅ Canonical JSON:");
            println!("{}", canonical);
            
            // Also show hash
            let mut chain = HashChain::new();
            let hash = chain.append(&json);
            println!("🔒 Hash: {}", hash);
        }
        
        Commands::HashChain { files } => {
            println!("⛓️  SEL Hash Chain (Day 2)");
            println!("📁 Files: {}", files.join(", "));
            println!("{}", "=".repeat(50));
            
            let mut chain = HashChain::new();
            
            for file in &files {
                let content = fs::read_to_string(file)?;
                let json: Value = serde_json::from_str(&content)?;
                
                let hash = chain.append(&json);
                println!("📄 {} → {}", file, hash);
            }
            
            println!();
            println!("📊 Summary:");
            println!("  • Files processed: {}", files.len());
            println!("  • Final hash: {}", chain.append(&json!("end")));
        }
        
        Commands::Validate { mission_file, verbose } => {
            println!("🛡️  SEL Validator (Day 4)");
            println!("📄 File: {}", mission_file);
            println!("{}", "=".repeat(50));
            
            let content = fs::read_to_string(&mission_file)?;
            let mission: Value = serde_json::from_str(&content)?;
            
            let mut validator = Validator::new();
            let result = validator.validate(&mission);
            
            match result {
                ValidationResult::Valid(validated) => {
                    println!("✅ VALIDATION PASSED");
                    println!();
                    
                    println!("📋 Summary:");
                    println!("  • Validator: {}", validated.validator_version());
                    println!("  • Mode: {:?}", validated.workspace_mode());
                    println!("  • Timestamp: {}", validated.validated_at());
                    
                    if verbose {
                        println!();
                        println!("🔐 Details:");
                        println!("  • Validation Proof: {}", validated.validation_proof());
                        println!("  • Allowed Commands: {:?}", validated.capabilities().allowed_commands);
                        println!("  • Max Execution Time: {} seconds", 
                                 validated.capabilities().max_execution_time.as_secs());
                    }
                }
                
                ValidationResult::Invalid { errors, suggestions } => {
                    println!("❌ VALIDATION FAILED");
                    println!();
                    
                    println!("🚫 Errors found:");
                    for (i, error) in errors.iter().enumerate() {
                        println!("  {}. [{}] {}", i + 1, error.error_type, error.message);
                        if let Some(location) = &error.location {
                            println!("     Location: {}", location);
                        }
                    }
                    
                    if !suggestions.is_empty() {
                        println!();
                        println!("💡 Suggestions:");
                        for suggestion in suggestions {
                            println!("  • {}", suggestion);
                        }
                    }
                    
                    std::process::exit(1);
                }
            }
        }
        
        Commands::Test { component } => {
            println!("🧪 SEL Integration Test");
            println!("{}", "=".repeat(50));
            
            match component.as_str() {
                "all" => {
                    println!("Running all tests...");
                    
                    // Test canonical JSON
                    println!("\n1️⃣  Testing canonical JSON...");
                    let test_json = json!({
                        "name": "test",
                        "actions": [{"type": "command", "command": "echo", "args": ["hello"]}]
                    });
                    let canonical = canonicalize_json(&test_json);
                    println!("   📝 Result: {}", canonical);
                    println!("   ✅ Canonical test passed");
                    
                    // Test hash chain
                    println!("\n2️⃣  Testing hash chain...");
                    let mut chain = HashChain::new();
                    let data1 = json!({"a": 1});
                    let data2 = json!({"b": 2});
                    
                    let hash1 = chain.append(&data1);
                    let hash2 = chain.append(&data2);
                    
                    println!("   🔗 Hash 1: {}", hash1);
                    println!("   🔗 Hash 2: {}", hash2);
                    println!("   ✅ Hash chain test passed");
                    
                    // Test validator
                    println!("\n3️⃣  Testing validator...");
                    
                    // Test valid mission
                    let valid_mission = json!({
                        "name": "test-mission",
                        "actions": [{"type": "command", "command": "echo", "args": ["hello"]}]
                    });
                    
                    let mut validator = Validator::new();
                    let result = validator.validate(&valid_mission);
                    
                    if let ValidationResult::Valid(_) = result {
                        println!("   ✅ Valid mission test passed");
                    } else {
                        println!("   ❌ Valid mission test failed");
                    }
                    
                    // Test forbidden command
                    let dangerous_mission = json!({
                        "name": "dangerous",
                        "actions": [{"type": "command", "command": "rm", "args": ["-rf", "/"]}]
                    });
                    
                    let result = validator.validate(&dangerous_mission);
                    if let ValidationResult::Invalid { .. } = result {
                        println!("   ✅ Forbidden command test passed");
                    } else {
                        println!("   ❌ Forbidden command test failed");
                    }
                    
                    println!("\n🎉 All tests completed successfully!");
                }
                
                "canonical" => {
                    println!("\n1️⃣  Testing canonical JSON...");
                    let test_json = json!({
                        "name": "test",
                        "actions": [{"type": "command", "command": "echo", "args": ["hello"]}]
                    });
                    let canonical = canonicalize_json(&test_json);
                    println!("   📝 Result: {}", canonical);
                    println!("   ✅ Canonical test passed");
                }
                
                "hash" => {
                    println!("\n2️⃣  Testing hash chain...");
                    let mut chain = HashChain::new();
                    let data1 = json!({"a": 1});
                    let data2 = json!({"b": 2});
                    
                    let hash1 = chain.append(&data1);
                    let hash2 = chain.append(&data2);
                    
                    println!("   🔗 Hash 1: {}", hash1);
                    println!("   🔗 Hash 2: {}", hash2);
                    println!("   ✅ Hash chain test passed");
                }
                
                "validate" => {
                    println!("\n3️⃣  Testing validator...");
                    
                    // Test valid mission
                    let valid_mission = json!({
                        "name": "test-mission",
                        "actions": [{"type": "command", "command": "echo", "args": ["hello"]}]
                    });
                    
                    let mut validator = Validator::new();
                    let result = validator.validate(&valid_mission);
                    
                    if let ValidationResult::Valid(_) = result {
                        println!("   ✅ Valid mission test passed");
                    } else {
                        println!("   ❌ Valid mission test failed");
                    }
                    
                    // Test forbidden command
                    let dangerous_mission = json!({
                        "name": "dangerous",
                        "actions": [{"type": "command", "command": "rm", "args": ["-rf", "/"]}]
                    });
                    
                    let result = validator.validate(&dangerous_mission);
                    if let ValidationResult::Invalid { .. } = result {
                        println!("   ✅ Forbidden command test passed");
                    } else {
                        println!("   ❌ Forbidden command test failed");
                    }
                }
                
                _ => {
                    println!("❌ Unknown component: {}", component);
                    println!("   Available: canonical, hash, validate, all");
                }
            }
        }
    }

    Ok(())
}
