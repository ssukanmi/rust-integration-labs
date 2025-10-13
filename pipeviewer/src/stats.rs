use std::{
    io::Result,
    sync::mpsc::{Receiver, Sender},
};

pub fn stats_loop(
    silent: bool,
    stats_rx: Receiver<Vec<u8>>,
    write_tx: Sender<Vec<u8>>,
) -> Result<()> {
    let mut total_bytes = 0;

    while let Ok(buffer) = stats_rx.recv() {
        // todo: receive bytes from read thread
        // let buffer = stats_rx.recv().unwrap();

        // let buffer: Vec<u8> = if let Ok(buffer) = stats_rx.recv() {
        //     buffer
        // } else {
        //     break;
        // };

        let num_bytes = buffer.len();

        // let buffer = Vec::<u8>::new();
        // let buffer: Vec<u8> = Vec::new();

        total_bytes += buffer.len();
        if !silent {
            eprint!("\r{total_bytes}");
        }

        // todo: send vector to write thread
        if write_tx.send(buffer).is_err() {
            break;
        };

        if num_bytes == 0 {
            break;
        }

        // let quit = quit.lock().unwrap();
        // if let Ok(quit) = quit.lock()
        //     && *quit
        // {
        //     break;
        // }
    }

    if !silent {
        eprintln!();
    }
    Ok(())
}

pub fn stats(silent: bool, num_read: usize, total_bytes: &mut usize, last: bool) {
    *total_bytes += num_read;

    if !silent {
        eprint!("\r{total_bytes}");
        if last {
            eprintln!();
        }
    }
}
