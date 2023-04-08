mod account;
mod io;

use anyhow::{Context, Result};
use clap::Parser;
use std::fs;

/// Cli arguments structure.
#[derive(Parser)]
struct Cli {
    /// The path to the file to read.
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let file_content = fs::read_to_string(&args.path)
        .with_context(|| format!("Could not read file '{}'", &args.path.to_str().unwrap()))?;

    let results = io::parse_file(file_content).with_context(|| {
        format!(
            "Could not parse file operation for file '{}'",
            &args.path.to_str().unwrap()
        )
    })?;

    results
        .into_iter()
        .for_each(|r| println!("{}", serde_json::to_string_pretty(&r).unwrap()));

    Ok(())
}
