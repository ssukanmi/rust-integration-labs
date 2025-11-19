use progress_iter::ProgressIteratorExt;
use std::{thread::sleep, time::Duration};

fn main() {
    println!("Unbounded progress (infinite iterator - commented out):");
    // for _ in (0..).progress() {
    //     sleep(Duration::from_secs(1));
    // }

    println!("Bounded progress with custom delimiters:");
    let v = [1, 2, 3];

    for _ in v.iter().progress().with_bound().with_delims(('|', '|')) {
        sleep(Duration::from_secs(1));
    }

    println!("\nDone!");
}
