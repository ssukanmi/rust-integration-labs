#![allow(clippy::unwrap_used)] // Dev purpose
use std::{
    env,
    error::Error,
    io::{self, ErrorKind, Read, Write},
};

use clap::Parser;

const CHUNK_SIZE: usize = 16 * 1024;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Read from a file instead of stdin
    infile: Option<String>,
    /// Write output to a file instead of stdout
    #[arg(short, long)]
    outfile: Option<String>,
    #[arg(short, long)]
    silent: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let infile = args.infile;
    let outfile = args.outfile;
    let silent = if args.silent {
        true
    } else {
        !env::var("PV_SILENT").unwrap_or_default().is_empty()
    };

    dbg!(infile, outfile, silent);

    let mut total_bytes = 0;
    let mut buffer = [0; CHUNK_SIZE];
    loop {
        if !silent {
            eprint!("\r{total_bytes}");
        }
        let num_read = match io::stdin().read(&mut buffer) {
            Ok(0) => break,
            Ok(x) => x,
            Err(_) => break,
        };
        total_bytes += num_read;
        if let Err(e) = io::stdout().write_all(&buffer[..num_read]) {
            if e.kind() == ErrorKind::BrokenPipe {
                break;
            }
            return Err(Box::new(e));
        }
    }

    if !silent {
        eprintln!("\r{total_bytes}");
    }

    Ok(())
}
