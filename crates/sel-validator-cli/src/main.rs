use clap::{Parser, Subcommand};
use sel_validator::{Validator, ValidationResult};
use serde_json::Value;
use std::process;

#[derive(Parser)]
#[command(author, version, about = "SEL Validator - Day 4: The Shield")]
struct Cli {
    /// Mission JSON file to validate
    #[arg(short, long)]
    file: String,
    
    /// Show detailed validation report
    #[arg(short, long)]
    verbose: bool,
}

fn main() {
    let cli = Cli::parse();
    
    println!("🛡️  SEL Validator - Day 4: The Shield");
    println!("📄 File: {}", cli.file);
    println!("{}", "=".repeat(50));
    
    // Read and parse JSON
    let content = match std::fs::read_to_string(&cli.file) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("❌ Failed to read file: {}", e);
            process::exit(1);
        }
    };
    
    let mission: Value = match serde_json::from_str(&content) {
        Ok(mission) => mission,
        Err(e) => {
            eprintln!("❌ Invalid JSON: {}", e);
            process::exit(1);
        }
    };
    
    // Validate
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
            
            if cli.verbose {
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
            
            process::exit(1);
        }
    }
}
