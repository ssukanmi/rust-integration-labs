use clap::Parser;
use regex::Regex;
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};

/// Searches for patterns")
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The pattern to search for
    pattern: String,

    /// File to search
    input: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let Args { pattern, input } = Args::parse();

    let re = Regex::new(&pattern)?;

    let reader: Box<dyn BufRead> = {
        if let Some(file) = input {
            Box::new(BufReader::new(File::open(file)?))
        } else {
            Box::new(BufReader::new(io::stdin().lock()))
        }
    };

    process_lines(reader, re);

    Ok(())
}

fn process_lines<T>(reader: T, re: Regex)
where
    T: BufRead + Sized,
{
    for line_ in reader.lines() {
        if let Ok(line) = line_
            && re.find(&line).is_some()
        {
            println!("{line}")
        }
    }
}
