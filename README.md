# 📌 Milestone 1: Read a Markdown File
##  Goal: Read a file path from CLI and print its content.

##  Steps:

    Use clap to parse CLI args (take a file path as input).

        Hint: Derive Parser for a struct with file_path: PathBuf.

    Read the file with std::fs::read_to_string.

        Handle errors (e.g., file not found) with anyhow.

    Print the raw content to verify it works.

##  Key Concepts:

    CLI argument parsing with clap.

    File I/O and error propagation (? operator).

# 📌 Milestone 2: Count Words (Excluding Code Blocks)
##  Goal: Count words outside code blocks (between ```).

##  Steps:

    Use regex::Regex to remove code blocks:
    rust

    let re = Regex::new(r"```.*?```").unwrap();  // Non-greedy match
    let text_without_code = re.replace_all(&content, "");

    Split the remaining text into words (split_whitespace()).

    Print the word count.

##  Key Concepts:

    Regular expressions with the regex crate.

    String manipulation.

# 📌 Milestone 3: Extract Headers
##  Goal: Count headers (#  Header).

##  Steps:

    Use regex to find headers:
    rust

    let re = Regex::new(r"(?m)^# {1,6}\s+(.+)$").unwrap();  // Multiline match

    Capture header text (e.g., #  Hello → Hello).

    Store headers in a Vec<String> and print their count.

##  Key Concepts:

    Regex capture groups (captures_iter).

    Multiline regex ((?m) flag).

# 📌 Milestone 4: Count Links and Code Blocks
##  Goal: Count [text](url) links and ``` code blocks.

##  Steps:

    Links: Use regex r"\[.*?\]\(.*?\)".

    Code Blocks: Use regex r"```.*?```" (count occurrences).

    Print the counts.

##  Key Concepts:

    Overlapping vs. non-overlapping matches.

# 📌 Milestone 5: Pretty-Print Statistics
##  Goal: Format output like this:
sh

##  📊 Markdown Analysis:
- Words: 420
- Headers: 5 (h1: 1, h2: 3, h3: 1)
- Code Blocks: 2
- Links: 8

##  Steps:

    Create a struct MarkdownStats to hold all metrics.

    Implement Display for pretty-printing (or just use println!).

# 📌 Milestone 6 (Bonus): JSON Output
##  Goal: Add --json flag to output stats as JSON.

##  Steps:

    Add a json: bool field to your CLI struct.

    Use serde_json::to_string_pretty when json is true.

##  🔍 Debugging Tips

    Test regex patterns interactively: regex101.com.

    Use dbg!(variable) to inspect values at runtime.

    Handle errors gracefully:
    rust

    let content = fs::read_to_string(&args.file_path)
        .map_err(|e| anyhow!("Failed to read {}: {}", args.file_path.display(), e))?;

##  🚀 Stretch Goals

    Support multiple files (e.g., md-analyzer ./dir/*.md).

    Add colors to output (colored crate).

    Measure code block lines (e.g., total_lines: 50).