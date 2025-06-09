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

    // println!("File content:\n{}", &content);

    let word_count = count_words(&content);
    let extracted_header = extract_headers(&content);
    let links_count = count_urls(&content);
    let code_block_count = count_code_blocks(&content);

    print_stats(word_count, &extracted_header, code_block_count, links_count);

    Ok(())
}

fn remove_code_block(content: &str) -> String {
    let re = Regex::new(r"(?s)```.*?```").unwrap();
    re.replace_all(content, "").to_string()
}

fn count_words(content: &str) -> usize {
    let cleared_context = remove_code_block(content);
    cleared_context.split_whitespace().count()
}

fn extract_headers(content: &str) -> Vec<(usize, String)> {
    let re = Regex::new(r"(?m)#{1,6}\s+(.+)$").unwrap();
    re.captures_iter(content).map(|cap| {
        let level = cap[0].chars().take_while(|x| *x == '#').count();
        let text = cap[1].trim().to_string();
        (level, text)
    }).collect()
}

fn count_urls(content: &str) -> usize {
    let re = Regex::new(r"(?m)\[(.+)\]\((.+)\)").unwrap();
    re.find_iter(content).count()
}

fn count_code_blocks(content: &str) -> usize {
    let re = Regex::new(r"(?s)```.*?```").unwrap();
    re.find_iter(content).count()
}

fn print_stats(words_count: usize, headers: &[(usize, String)], code_blocks: usize, links: usize) {
    println!("📊 Markdown Analysis");
    println!("----------------------");

    println!(". Words: {}", words_count);

    let mut headers_count = [0;6];
    for (level, _) in headers {
        if *level >= 1 && *level <= 6 {
            headers_count[*level - 1] += 1;
        }
    }
    println!(". Headers:");
    for (level, count) in headers_count.iter().enumerate() {
        println!("\t- h{}: {}", level+1, count);
    }
    println!(". Code blocks: {}", code_blocks);
    println!(". Links: {}", links);

}