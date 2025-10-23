use rand::{Rng, rng};
use std::{error::Error, fmt};

#[derive(Debug, PartialEq)]
enum FileState {
    Open,
    Closed,
}

impl fmt::Display for FileState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FileState::Open => write!(f, "OPEN"),
            FileState::Closed => write!(f, "CLOSED"),
        }
    }
}

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
    state: FileState,
}

trait Read {
    fn read(&self, save_to: &mut Vec<u8>) -> Result<usize, String>;
}

impl Read for File {
    fn read(&self, save_to: &mut Vec<u8>) -> Result<usize, String> {
        if self.state != FileState::Open {
            return Err(String::from("File must be open for reading"));
        }
        let mut tmp = self.data.clone();
        let read_len = tmp.len();

        save_to.reserve(read_len);
        save_to.append(&mut tmp);
        Ok(read_len)
    }
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<{} {}>", self.name, self.state)
    }
}

impl File {
    fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Closed,
        }
    }

    fn new_with_data(name: &str, data: &[u8]) -> Self {
        let mut f = Self::new(name);
        f.data = data.to_vec();
        f
    }
}

fn open(mut f: File) -> Result<File, String> {
    if one_in(100) {
        return Err(String::from("Permission denied"));
    }
    f.state = FileState::Open;
    Ok(f)
}

fn close(mut f: File) -> Result<File, String> {
    if one_in(100) {
        return Err(String::from("Interrupted by signal!"));
    }
    f.state = FileState::Closed;
    Ok(f)
}

fn one_in(denominator: u32) -> bool {
    rng().random_ratio(1, denominator)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut f2 = File::new_with_data("f2.txt", &[114, 117, 115, 116, 33]);

    let mut buffer = vec![];

    f2 = open(f2)?;

    let f2_len = f2.read(&mut buffer)?;

    f2 = close(f2)?;

    let txt = String::from_utf8_lossy(&buffer);

    println!("{}", f2);
    println!("{:?}", f2);
    println!("{} is {} bytes long", &f2.name, f2_len);
    println!("{}", txt);

    Ok(())
}
