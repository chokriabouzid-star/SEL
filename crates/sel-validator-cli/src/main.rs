//! SEL Validator CLI - Core 1.0
//! Command-line interface for mission validation

use clap::{Parser, Subcommand};
use sel_common::SovereignError;
use sel_validator::{ValidatedMission, ValidationConfig, Validator};
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Validate a mission file
    Validate {
        /// Path to mission JSON file
        mission: PathBuf,

        /// Maximum actions allowed
        #[arg(short, long, default_value_t = 1000)]
        max_actions: usize,

        /// Disable strict security mode
        #[arg(long)]
        no_strict: bool,
    },
}

fn main() -> Result<(), SovereignError> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Validate {
            mission,
            max_actions,
            no_strict,
        } => {
            validate_mission(&mission, max_actions, !no_strict)?;
        }
    }

    Ok(())
}

fn validate_mission(
    path: &PathBuf,
    max_actions: usize,
    strict_mode: bool,
) -> Result<ValidatedMission, SovereignError> {
    println!("🔐 SEL Validator Core 1.0");
    println!("========================");
    println!("📄 Mission: {}", path.display());

    // Read mission file
    let content = fs::read_to_string(path)
        .map_err(|e| SovereignError::InvalidMissionFormat(format!("Failed to read file: {}", e)))?;

    // Create validator
    let config = ValidationConfig {
        max_actions,
        strict_mode,
        ..Default::default()
    };

    let validator = Validator::new(config);

    // Validate mission
    println!("🔍 Validating...");
    let validated = validator.validate(&content)?;

    println!("✅ VALIDATION SUCCESSFUL");
    println!("   • Validator: {}", validated.validator_version());
    println!("   • Workspace Mode: {:?}", validated.workspace_mode());
    println!("   • Actions: {}", validated.actions().len());

    // ✅ إصلاح clippy warning: 16.min(64) → 16
    let proof = validated.validation_proof_str();
    let truncated = if proof.len() > 16 {
        &proof[..16]
    } else {
        proof
    };
    println!("   • Proof: {}...", truncated);

    Ok(validated)
}
