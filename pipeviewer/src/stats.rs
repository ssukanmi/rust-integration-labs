use std::{
    io::Result,
    sync::{Arc, Mutex},
};

pub fn stats_loop(silent: bool, quit: Arc<Mutex<bool>>) -> Result<()> {
    let mut total_bytes = 0;

    loop {
        // todo: receive bytes from read thread
        // let buffer = Vec::<u8>::new();
        let buffer: Vec<u8> = Vec::new();
        total_bytes += buffer.len();
        if !silent {
            eprint!("\r{total_bytes}");
        }
        // todo: send vector to write thread
        // let quit = quit.lock().unwrap();
        if let Ok(quit) = quit.lock()
            && *quit
        {
            break;
        }
    }
    eprintln!();
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
