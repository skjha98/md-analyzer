use clap::Parser;
use std::{fs, path::PathBuf};
use anyhow::{Context, Result};

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    /// Path to Markdown json
    file_path: PathBuf,
}

fn main() -> Result<()>{
    let args = Args::parse();

    let content = fs::read_to_string(&args.file_path).with_context(|| format!("Failed to read {}", args.file_path.display()))?;

    println!("File content:\n{}", content);
    Ok(())
}
