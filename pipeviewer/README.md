# Pipeviewer

A command-line utility for monitoring data throughput in pipelines, similar to the classic `pv` tool. Displays real-time progress statistics including bytes transferred, elapsed time, and transfer rate with colored terminal output.

## Features

- **Real-time progress display** - Monitor bytes transferred, elapsed time (HH:MM:SS), and transfer rate (bytes/sec)
- **Colored output** - Easy-to-read statistics with color-coded information (red for bytes, green for time, blue for rate)
- **Flexible I/O** - Read from stdin or file, write to stdout or file
- **Multi-threaded architecture** - Uses crossbeam channels for efficient parallel read-write operations
- **Silent mode** - Suppress progress output via flag or environment variable

## Installation

```bash
cargo install --path .
```

## Usage

### Basic Examples

```bash
# Monitor data flowing through pipes
cat large_file.txt | pipeviewer > output.txt

# Read from file
pipeviewer input.txt > output.txt

# Write to specific output file
cat input.txt | pipeviewer -o output.txt

# Silent mode (no progress output)
pipeviewer -s input.txt > output.txt

# Using environment variable for silent mode
PV_SILENT=true pipeviewer input.txt > output.txt
```

### Real-world Examples

```bash
# Monitor file compression
pipeviewer large_file.txt | gzip > large_file.txt.gz

# Monitor download progress
curl https://example.com/file.zip | pipeviewer > file.zip

# Copy files with progress tracking
pipeviewer large_file.iso -o /media/usb/large_file.iso
```

## Options

- `[INFILE]` - Read from file instead of stdin
- `-o, --outfile <FILE>` - Write to file instead of stdout
- `-s, --silent` - Suppress progress output (can also use `PV_SILENT` environment variable)
- `-h, --help` - Print help information
- `-V, --version` - Print version information

## Architecture

Pipeviewer uses a multi-threaded architecture for optimal performance:

1. **Read thread** - Reads data from input source in 16KB chunks
2. **Stats thread** - Receives byte counts and displays progress statistics (updates every 100ms)
3. **Write thread** - Receives data buffers and writes to output destination

The threads communicate via crossbeam channels, allowing concurrent read-write operations while maintaining accurate progress tracking.
