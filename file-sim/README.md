# file-sim

A simple file system simulation with state management and error handling.

## Features

- File state management (Open/Closed)
- Error handling with random failures
- Read trait implementation
- Display formatting for files and states

## Usage

```rust
use file_sim::file::{self, File, Read};

let mut my_file = File::new_with_data("data.txt", b"hello");
let mut buffer = vec![];

my_file = file::open(my_file)?;
my_file.read(&mut buffer)?;
my_file = file::close(my_file)?;
```

## Running

```bash
cargo run -p file-sim
```

## Documentation

View the full documentation:

```bash
cargo doc -p file-sim --no-deps --open
```

## Dependencies

- **rand**: Random error simulation
