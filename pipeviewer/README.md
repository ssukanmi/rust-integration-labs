# Pipeviewer

A simple pipe viewer utility that monitors data flowing through pipes and displays the number of bytes transferred.

## Usage

```bash
# Monitor bytes as data flows through pipe
cat myfile | cargo run -p pipeviewer | gzip > output.gz

# Read from file
cargo run -p pipeviewer myfile

# Silent mode
cargo run -p pipeviewer --silent myfile
```

## Options

- `[INFILE]` - Read from file instead of stdin
- `-o, --outfile <FILE>` - Write to file instead of stdout
- `-s, --silent` - Suppress progress output
