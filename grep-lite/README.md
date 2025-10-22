# grep-lite

A simple pattern search tool, like Unix `grep`, written in Rust.

## What It Does

Searches for text patterns in files or from standard input and prints matching lines.

## Usage

```bash
# Search in a file
cargo run -- "pattern" filename.txt

# Search from stdin
echo "hello world" | cargo run -- "world"

# Use regex patterns
cargo run -- "^fn" src/main.rs
```

## Examples

```bash
# Find all function definitions
cargo run -- "fn " src/main.rs

# Find lines with numbers
cargo run -- "[0-9]+" data.txt

# Search Cargo.toml
cat Cargo.toml | cargo run -- "clap"
```

## How It Works

1. Takes a pattern and optional filename as arguments
2. Reads lines from the file (or stdin if no file given)
3. Prints lines that match the pattern using regex

## Dependencies

- **clap**: Command-line argument parsing
- **regex**: Pattern matching
