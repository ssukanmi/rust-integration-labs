use rand::{Rng, rng};
use std::fmt;

/// Represents the state of a file.
#[derive(Debug, PartialEq)]
pub enum FileState {
    /// File is open and ready for reading.
    Open,
    /// File is closed.
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

/// A simulated file with state management.
#[derive(Debug)]
pub struct File {
    /// The name of the file.
    pub name: String,
    /// Internal data storage.
    data: Vec<u8>,
    /// Current state of the file.
    pub state: FileState,
}

/// Trait for reading data from a file.
pub trait Read {
    /// Reads data from the file into the provided buffer.
    ///
    /// # Errors
    ///
    /// Returns an error if the file is not open.
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
    /// Creates a new empty file with the given name.
    ///
    /// The file is created in a closed state.
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Closed,
        }
    }

    /// Creates a new file with initial data.
    ///
    /// The file is created in a closed state.
    pub fn new_with_data(name: &str, data: &[u8]) -> Self {
        let mut f = Self::new(name);
        f.data = data.to_vec();
        f
    }
}

/// Opens a file for reading.
///
/// # Errors
///
/// Randomly returns "Permission denied" error (1% chance).
pub fn open(mut f: File) -> Result<File, String> {
    if one_in(100) {
        return Err(String::from("Permission denied"));
    }
    f.state = FileState::Open;
    Ok(f)
}

/// Closes an open file.
///
/// # Errors
///
/// Randomly returns "Interrupted by signal!" error (1% chance).
pub fn close(mut f: File) -> Result<File, String> {
    if one_in(100) {
        return Err(String::from("Interrupted by signal!"));
    }
    f.state = FileState::Closed;
    Ok(f)
}

fn one_in(denominator: u32) -> bool {
    rng().random_ratio(1, denominator)
}
