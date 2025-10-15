//! # Pipeviewer
//!
//! A pipe viewer utility that monitors data throughput in real-time.
//!
//! Pipeviewer displays progress statistics including bytes transferred, elapsed time,
//! and transfer rate when piping data between commands or files. It uses a multi-threaded
//! architecture with crossbeam channels for efficient parallel processing.
//!
//! ## Architecture
//!
//! The application uses three concurrent threads:
//! - **Read thread**: Reads data from input in chunks
//! - **Stats thread**: Processes and displays progress statistics
//! - **Write thread**: Writes data to output
//!
//! ## Example
//!
//! ```bash
//! # Monitor data flowing through pipes
//! cat large_file.txt | pipeviewer > output.txt
//!
//! # Silent mode (no progress output)
//! pipeviewer -s input.txt > output.txt
//! ```

/// Command-line argument parsing
pub mod args;
/// Reading data from input sources
pub mod read;
/// Progress statistics and display
pub mod stats;
/// Writing data to output destinations
pub mod write;

/// Size of buffer chunks for reading/writing data (16 KB)
pub const CHUNK_SIZE: usize = 16 * 1024;
