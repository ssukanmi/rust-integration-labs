//! # file-sim
//!
//! A simple file system simulation with state management and error handling.
//!
//! ## Example
//!
//! ```no_run
//! use file_sim::file::{self, File, Read};
//!
//! let mut my_file = File::new_with_data("data.txt", b"hello");
//! let mut buffer = vec![];
//!
//! my_file = file::open(my_file).unwrap();
//! my_file.read(&mut buffer).unwrap();
//! my_file = file::close(my_file).unwrap();
//! ```

pub mod file;
