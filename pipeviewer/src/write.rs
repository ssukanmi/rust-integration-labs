//! Writing data to output destinations.
//!
//! This module handles writing data to either stdout or a file,
//! receiving data from the read thread via a channel.

use crossbeam::channel::Receiver;
use std::{
    fs::File,
    io::{self, BufWriter, ErrorKind, Result, Write},
};

/// Continuously receives data from a channel and writes it to output.
///
/// # Arguments
///
/// * `outfile` - Optional path to output file. If `None`, writes to stdout.
/// * `writer_rx` - Channel receiver for data buffers from the read thread.
///
/// # Returns
///
/// Returns `Ok(())` on successful completion or an I/O error.
///
/// # Behavior
///
/// - Receives data buffers from the channel and writes them to output
/// - Stops when an empty buffer is received (EOF signal)
/// - Gracefully handles `BrokenPipe` errors (returns `Ok(())`)
pub fn write_loop(outfile: &Option<String>, writer_rx: Receiver<Vec<u8>>) -> Result<()> {
    let mut writer: Box<dyn Write> = {
        if let Some(file) = outfile {
            Box::new(BufWriter::new(File::create(file)?))
        } else {
            Box::new(BufWriter::new(io::stdout()))
        }
    };

    while let Ok(buffer) = writer_rx.recv() {
        if buffer.is_empty() {
            break;
        }

        if let Err(e) = writer.write_all(&buffer) {
            if e.kind() == ErrorKind::BrokenPipe {
                return Ok(());
            }
            return Err(e);
        }
    }

    Ok(())
}

/// Writes a buffer to output (legacy function).
///
/// # Arguments
///
/// * `outfile` - Optional path to output file. If `None`, writes to stdout.
/// * `buffer` - Data buffer to write.
///
/// # Returns
///
/// Returns `Ok(true)` on success, `Ok(false)` on broken pipe, or an I/O error.
///
/// # Note
///
/// This function is kept for compatibility but is not used in the current
/// multi-threaded implementation. Use [`write_loop`] instead.
pub fn write(outfile: &Option<String>, buffer: &[u8]) -> Result<bool> {
    let mut writer: Box<dyn Write> = {
        if let Some(file) = outfile {
            Box::new(BufWriter::new(File::create(file)?))
        } else {
            Box::new(BufWriter::new(io::stdout()))
        }
    };

    if let Err(e) = writer.write_all(buffer) {
        if e.kind() == ErrorKind::BrokenPipe {
            return Ok(false);
        }
        return Err(e);
    }

    Ok(true)
}
