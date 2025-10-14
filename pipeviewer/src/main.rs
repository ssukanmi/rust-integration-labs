use clap::Parser;
use crossbeam::channel;
use pipeviewer::{args::Args, read, stats, write};
use std::{error::Error, thread};

fn main() -> Result<(), Box<dyn Error>> {
    let Args {
        infile,
        outfile,
        silent,
    } = Args::parse();

    let (stats_tx, stats_rx) = channel::unbounded();
    let (write_tx, write_rx) = channel::bounded(1024);

    let read_handle = thread::spawn(move || read::read_loop(&infile, stats_tx, write_tx));
    let stats_handle = thread::spawn(move || stats::stats_loop(silent, stats_rx));
    let write_handle = thread::spawn(move || write::write_loop(&outfile, write_rx));

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

    // let args = Args::parse();
    // let mut _total_bytes = 0;
    //
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
