//! Command-line argument parsing for pipeviewer.
//!
//! This module defines the command-line interface using `clap`.

use clap::Parser;

/// Command-line arguments for pipeviewer
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Read from a file instead of stdin
    pub infile: Option<String>,

    /// Write output to a file instead of stdout
    #[arg(short, long)]
    pub outfile: Option<String>,

    /// Suppress progress output
    ///
    /// Can also be set via the `PV_SILENT` environment variable
    #[arg(short, long, env = "PV_SILENT")]
    pub silent: bool,
}
