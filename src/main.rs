use clap::Parser;
use std::{fs, path::PathBuf};
use anyhow::{Context, Result};
use regex::Regex;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    /// Path to Markdown json
    file_path: PathBuf,
}

fn main() -> Result<()>{
    let args = Args::parse();

    let content = fs::read_to_string(&args.file_path).with_context(|| format!("Failed to read {}", args.file_path.display()))?;

    println!("File content:\n{}", &content);

    let word_count = count_words(&content);

    println!("Word count:\n{}", word_count);
    Ok(())
}

fn remove_code_block(content: &str) -> String {
    let re = Regex::new("```.*?```").unwrap();
    re.replace_all(content, "").to_string()
}

fn count_words(content: &str) -> usize {
    let cleared_context = remove_code_block(content);
    cleared_context.split_whitespace().map(|x| println!("{}", x)).count()
}

