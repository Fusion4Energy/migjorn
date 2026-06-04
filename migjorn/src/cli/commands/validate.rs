use clap::Args;
use std::path::PathBuf;

use crate::cli::utils::load_model;

/// Parse a model file and perform validation checks
#[derive(Args)]
pub struct CheckArgs {
    /// Path to the MCNP input file
    pub file: PathBuf,
}

pub fn run(args: &CheckArgs) -> i32 {
    let model = load_model(&args.file);
    match model {
        Ok(m) => {
            println!("Model loaded successfully. Performing checks...");
            m.validation_checks().unwrap_or_else(|e| {
                eprintln!("Validation failed: {e}");
                std::process::exit(1);
            });
            println!("Validation passed.");
            0
        }
        Err(e) => {
            eprintln!("Failed to load model: {e}");
            1
        }
    }
}
