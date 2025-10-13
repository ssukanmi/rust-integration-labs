use std::{
    fs::File,
    io::{self, BufWriter, ErrorKind, Result, Write},
    sync::{Arc, Mutex},
};

pub fn write_loop(outfile: &Option<String>, quit: Arc<Mutex<bool>>) -> Result<()> {
    let mut writer: Box<dyn Write> = {
        if let Some(file) = outfile {
            Box::new(BufWriter::new(File::create(file)?))
        } else {
            Box::new(BufWriter::new(io::stdout()))
        }
    };

    loop {
        // todo: recieve bytes to write from stats
        let buffer: Vec<u8> = Vec::new();

        {
            // let quit = quit.lock().unwrap();
            if let Ok(quit) = quit.lock()
                && *quit
            {
                break;
            }
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
