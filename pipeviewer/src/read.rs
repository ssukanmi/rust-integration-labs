use crate::CHUNK_SIZE;
use std::{
    fs::File,
    io::{self, BufReader, Read, Result},
};

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
