//! Reading data from input sources.
//!
//! This module handles reading data from either stdin or a file,
//! and sending it to the stats and write threads via channels.

use crate::CHUNK_SIZE;
use crossbeam::channel::Sender;
use std::{
    fs::File,
    io::{self, BufReader, Read, Result},
};

/// Continuously reads data from input and sends it to stats and write channels.
///
/// # Arguments
///
/// * `infile` - Optional path to input file. If `None`, reads from stdin.
/// * `stats_tx` - Channel sender for byte counts to the stats thread.
/// * `write_tx` - Channel sender for data buffers to the write thread.
///
/// # Returns
///
/// Returns `Ok(())` on successful completion or an I/O error.
///
/// # Behavior
///
/// Reads data in chunks of [`CHUNK_SIZE`], sending:
/// - Byte count to stats thread for each chunk read
/// - Data buffer to write thread for output
/// - Empty signals (0 bytes and empty Vec) when input is exhausted
pub fn read_loop(
    infile: &Option<String>,
    stats_tx: Sender<usize>,
    write_tx: Sender<Vec<u8>>,
) -> Result<()> {
    let mut reader: Box<dyn Read> = {
        if let Some(file) = infile {
            Box::new(BufReader::new(File::open(file)?))
        } else {
            Box::new(BufReader::new(io::stdin()))
        }
    };

    let mut buffer = [0; CHUNK_SIZE];

    loop {
        let num_read = match reader.read(&mut buffer) {
            Ok(0) => break,
            Ok(x) => x,
            Err(_) => break,
        };
        let _ = stats_tx.send(num_read);
        if write_tx.send(Vec::from(&buffer[..num_read])).is_err() {
            break;
        }
    }

    let _ = stats_tx.send(0);
    let _ = write_tx.send(Vec::new());

    Ok(())
}

/// Reads a single chunk of data from input (legacy function).
///
/// # Arguments
///
/// * `infile` - Optional path to input file. If `None`, reads from stdin.
///
/// # Returns
///
/// Returns a `Vec<u8>` containing the data read, or an I/O error.
///
/// # Note
///
/// This function is kept for compatibility but is not used in the current
/// multi-threaded implementation. Use [`read_loop`] instead.
pub fn read(infile: &Option<String>) -> Result<Vec<u8>> {
    // let mut reader: Box<dyn Read> = if infile.is_some() {
    //     Box::new(File::open(infile.unwrap())?)
    // } else {
    //     Box::new(io::stdin())
    // };

    let mut reader: Box<dyn Read> = {
        if let Some(file) = infile {
            Box::new(BufReader::new(File::open(file)?))
        } else {
            Box::new(BufReader::new(io::stdin()))
        }
    };

    let mut buffer = [0; CHUNK_SIZE];
    let num_read = reader.read(&mut buffer)?;

    Ok(Vec::from(&buffer[..num_read]))
}
