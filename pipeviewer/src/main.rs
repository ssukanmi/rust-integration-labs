use clap::Parser;
use pipeviewer::{args::Args, read, stats, write};
use std::{
    error::Error,
    sync::{Arc, Mutex},
    thread,
};

fn main() -> Result<(), Box<dyn Error>> {
    // let args = Args::parse();
    let Args {
        infile,
        outfile,
        silent,
    } = Args::parse();

    let mut _total_bytes = 0;

    let quit = Arc::new(Mutex::new(false));
    let (quit1, quit2, quit3) = (Arc::clone(&quit), Arc::clone(&quit), Arc::clone(&quit));

    let read_handle = thread::spawn(move || read::read_loop(&infile, quit1));
    let stats_handle = thread::spawn(move || stats::stats_loop(silent, quit2));
    let write_handle = thread::spawn(move || write::write_loop(&outfile, quit3));

    let read_io_result = read_handle
        .join()
        .map_err(|e| format!("Read thread panicked: {:?}", e))?;
    let stats_io_result = stats_handle
        .join()
        .map_err(|e| format!("Stats thread panicked: {:?}", e))?;
    let write_io_result = write_handle
        .join()
        .map_err(|e| format!("Write thread panicked: {:?}", e))?;

    read_io_result?;
    stats_io_result?;
    write_io_result?;

    // loop {
    //     let buffer = match read::read(&args.infile) {
    //         Ok(x) if x.is_empty() => break,
    //         Ok(x) => x,
    //         Err(_) => break,
    //     };

    //     stats::stats(args.silent, buffer.len(), &mut total_bytes, false);

    //     if !write::write(&args.outfile, &buffer)? {
    //         break;
    //     }
    // }
    // stats::stats(args.silent, 0, &mut total_bytes, true);

    Ok(())
}
