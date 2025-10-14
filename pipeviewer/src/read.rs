use crate::CHUNK_SIZE;
use crossbeam::channel::Sender;
use std::{
    fs::File,
    io::{self, BufReader, Read, Result},
};

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
