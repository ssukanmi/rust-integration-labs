# Rust Integration Labs

A Rust workspace for learning and experimenting with various Rust concepts, from low-level bit manipulation to web services and database integrations.

## Projects

### Low-Level & Systems Programming

- **[bit-patterns](bit-patterns/)** - Explore binary representations of floats (IEEE 754), fixed-point numbers (Q7), and bit manipulation
- **[file-sim](file-sim/)** - File system simulation demonstrating error handling and traits
- **[rust-in-action](rust-in-action/)** - Examples from "Rust in Action" book

### Command-Line Tools

- **[grep-lite](grep-lite/)** - A lightweight grep implementation
- **[pipeviewer](pipeviewer/)** - Pipeline data viewer with progress indicators
- **[mandelbrot](mandelbrot/)** - Mandelbrot set fractal generator

### Web & Backend

- **[axum-webservice](axum-webservice/)** - REST API using Axum web framework
- **[rust-jwt](rust-jwt/)** - JWT authentication implementation
- **[rust-sqlite](rust-sqlite/)** - SQLite database integration
- **[valkey-stream](valkey-stream/)** - Redis/Valkey streaming examples
- **[rust-job](rust-job/)** - Job processing system

## Getting Started

```bash
# Build the entire workspace
cargo build

# Run a specific project
cargo run -p bit-patterns
cargo run -p grep-lite
cargo run -p file-sim

# Run tests for all projects
cargo test

# Run tests for a specific project
cargo test -p bit-patterns
```

## Requirements

- Rust 1.80+ (uses 2024 edition)

## Workspace Configuration

This workspace uses:
- Cargo workspace resolver version 3
- Shared dependencies for common crates
- Workspace-level lints:
  - `unsafe_code = "forbid"` - No unsafe code allowed
  - `unwrap_used = "warn"` - Warning on `.unwrap()` usage
  - `expect_used = "warn"` - Warning on `.expect()` usage

## License

MIT
