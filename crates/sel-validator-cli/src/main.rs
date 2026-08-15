//! SEL Validator CLI - Core 1.1 / 1.2
//! Mission validation (HMAC + Ed25519) and independent proof verification.

use clap::{Parser, Subcommand};
use sel_common::{canonicalize_json, SovereignError};
use sel_validator::{Ed25519Authority, ValidatedMission, ValidationConfig, Validator};
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
    /// Validate a mission file (signs with HMAC + Ed25519)
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

    /// Independently verify an Ed25519 proof against a mission file.
    ///
    /// Requires only the mission file, the Ed25519 signature, and the
    /// signer's public key — the three values printed by `validate`.
    /// Does NOT require the HMAC secret or the Ed25519 private key.
    /// Safe to run on any machine, by any party (including auditors).
    Verify {
        /// Path to the mission JSON file being verified
        #[arg(long)]
        mission: PathBuf,

        /// Ed25519 signature hex (printed by `validate`)
        #[arg(long)]
        signature: String,

        /// Ed25519 public key hex of the signer (printed by `validate`)
        #[arg(long)]
        pubkey: String,
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
        Commands::Verify {
            mission,
            signature,
            pubkey,
        } => {
            verify_proof(&mission, &signature, &pubkey)?;
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

    let content = fs::read_to_string(path)
        .map_err(|e| SovereignError::InvalidMissionFormat(format!("Failed to read file: {}", e)))?;

    let config = ValidationConfig {
        max_actions,
        strict_mode,
    };

    let validator = Validator::new(config);

    if !strict_mode {
        eprintln!(
            "⚠️  WARNING: --no-strict disables path-traversal and dangerous-pattern              checks. The proof produced is cryptographically distinct from a              strict-mode proof and is marked strict_mode=false in the signed payload."
        );
    }

    println!("🔍 Validating...");
    let validated = validator.validate(&content)?;

    println!("✅ Mission validated successfully");
    println!("   • Validator:      {}", validated.validator_version());
    println!("   • Strict Mode:    {}", validated.strict_mode());
    println!("   • Workspace Mode: {:?}", validated.workspace_mode());
    println!("   • Actions:        {}", validated.actions().len());
    println!("   • Hash:  sel:v1.0:sha256:{}", validated.mission_hash());
    println!("   • Proof (HMAC):   {}", validated.validation_proof_str());

    if let (Some(sig), Some(pubkey)) = (
        validated.ed25519_proof_str(),
        validated.ed25519_public_key_str(),
    ) {
        println!("   • Proof (Ed25519):    {}", sig);
        println!("   • Ed25519 Public Key: {}", pubkey);
        println!("   ℹ️  Verify independently (no secret needed):");
        println!(
            "      sel-validator-cli verify --mission {} --signature {} --pubkey {}",
            path.display(),
            sig,
            pubkey
        );
    }

    Ok(validated)
}

fn verify_proof(
    mission_path: &PathBuf,
    signature: &str,
    pubkey: &str,
) -> Result<(), SovereignError> {
    println!("🔎 SEL Independent Verification (Ed25519)");
    println!("==========================================");
    println!("📄 Mission: {}", mission_path.display());

    let content = fs::read_to_string(mission_path)
        .map_err(|e| SovereignError::InvalidMissionFormat(format!("Failed to read file: {}", e)))?;

    // Recanonicalize independently — do not trust any cached state
    let canonical = canonicalize_json(&content)
        .map_err(|e| SovereignError::InvalidMissionFormat(e.to_string()))?;

    let verifier = Ed25519Authority::verifier_from_public_hex(pubkey)
        .map_err(|e| SovereignError::InvalidMissionFormat(format!("Invalid public key: {}", e)))?;

    match verifier.verify(canonical.as_bytes(), signature) {
        Ok(()) => {
            println!("✅ VALID — signature matches this mission and public key.");
            Ok(())
        }
        Err(_) => {
            println!(
                "❌ INVALID — signature does NOT match.\n\
                 The mission may have been modified after signing, or the\n\
                 signature/public key belongs to a different validator."
            );
            Err(SovereignError::ValidationFailed(
                "Ed25519 verification failed".to_string(),
            ))
        }
    }
}
