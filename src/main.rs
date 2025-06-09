use clap::Parser;
use std::{fs, path::PathBuf};
use anyhow::{Context, Result};
use regex::Regex;
use serde::Serialize;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    /// Path to Markdown
    file_path: PathBuf,

    /// Output as json
    #[arg(short, long)]
    json: bool
}

#[derive(Serialize)]
struct MarkdownStats {
    word_count: usize,
    headers: Vec<(usize, String)>,
    code_blocks: usize,
    links: usize,
}

fn main() -> Result<()>{
    let args = Args::parse();

    let content = fs::read_to_string(&args.file_path).with_context(|| format!("Failed to read {}", args.file_path.display()))?;

    if content.trim().is_empty() {
        println!("⚠️ File is empty");
        return Ok(());
    }

    // println!("File content:\n{}", &content);

    let stats: MarkdownStats = MarkdownStats {
        word_count: count_words(&content),
        headers: extract_headers(&content),
        code_blocks: count_code_blocks(&content),
        links: count_urls(&content),
    };

    if args.json == true {
        println!("{}", serde_json::to_string_pretty(&stats)?);
    }
    else {
        print_stats(stats.word_count, &stats.headers, stats.code_blocks, stats.links);
    }

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

fn print_stats(word_count: usize, headers: &[(usize, String)], code_blocks: usize, links: usize) {
    println!("📊 Markdown Analysis");
    println!("----------------------");

    println!(". Words: {}", word_count);

    let mut headers_count = [0;6];
    for (level, _) in headers {
        if *level >= 1 && *level <= 6 {
            headers_count[*level - 1] += 1;
        }
    }
    
    if headers.is_empty() {
        println!(". Headers: None");
    }
    else {
        println!(". Headers:");
        for (level, count) in headers_count.iter().enumerate() {
            if *count > 0 {
                println!("\t- h{}: {}", level+1, count);
            }
        }
    }
    println!(". Code blocks: {}", code_blocks);
    println!(". Links: {}", links);

}