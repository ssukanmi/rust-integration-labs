use crossbeam::channel::Receiver;
use std::{io::Result, time::{Duration, Instant}};

trait TimeOutput {
    fn as_time(&self) -> String;
}

impl TimeOutput for u64 {
    fn as_time(&self) -> String {
        let (hours, left) = (*self/3600, *self%3600);
        let (mins, secs) = (left/60, left%60);
        format!("{}:{:02}:{:02}", hours, mins, secs)
    }
}

struct Timer {
    last_instant: Instant,
    delta: Duration,
    period: Duration,
    count_down: Duration,
    ready: bool,
}

impl Timer {
    fn new() -> Self {
        let now = Instant::now();
        Self {
            last_instant: now,
            delta: Duration::default(),
            period: Duration::from_millis(100),
            count_down: Duration::default(),
            ready: true,
        }
    }

    fn update(&mut self) {
        let now = Instant::now();
        self.delta = now - self.last_instant;
        self.last_instant = now;
        self.count_down = self.count_down.checked_sub(self.delta).unwrap_or_else(|| {
            self.ready = true;
            self.period
        });
    }
}

pub fn stats_loop(silent: bool, stats_rx: Receiver<usize>) -> Result<()> {
    let mut total_bytes = 0;
    let start = Instant::now();
    let mut timer = Timer::new();

    while let Ok(num_bytes) = stats_rx.recv() {
        timer.update();
        let rate_per_second = num_bytes as f64 / timer.delta.as_secs_f64();
        total_bytes += num_bytes;
        if !silent && timer.ready {
            timer.ready = false;
            eprint!(
                "\r{} {} [{:.0}b/s]",
                total_bytes,
                start.elapsed().as_secs().as_time(),
                rate_per_second
            );
        }

        if num_bytes == 0 {
            break;
        }
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
