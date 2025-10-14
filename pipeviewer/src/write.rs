use crossbeam::channel::Receiver;
use std::{
    fs::File,
    io::{self, BufWriter, ErrorKind, Result, Write},
};

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
