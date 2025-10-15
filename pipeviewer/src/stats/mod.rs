//! Progress statistics and display.
//!
//! This module handles receiving byte counts from the read thread,
//! calculating statistics, and displaying progress with colored output.

mod timer;

use crossbeam::channel::Receiver;
use crossterm::{
    cursor, execute,
    style::{self, Color, PrintStyledContent, Stylize},
    terminal::{Clear, ClearType},
};
use std::{
    io::{self, Result, Stderr, Write},
    time::Instant,
};

use crate::stats::timer::Timer;

/// Trait for formatting seconds as HH:MM:SS time format.
trait TimeOutput {
    /// Converts seconds to a formatted time string.
    fn as_time(&self) -> String;
}

impl TimeOutput for u64 {
    fn as_time(&self) -> String {
        let (hours, left) = (*self / 3600, *self % 3600);
        let (mins, secs) = (left / 60, left % 60);
        format!("{}:{:02}:{:02}", hours, mins, secs)
    }
}

/// Continuously receives byte counts and displays progress statistics.
///
/// # Arguments
///
/// * `silent` - If true, suppresses all progress output.
/// * `stats_rx` - Channel receiver for byte counts from the read thread.
///
/// # Returns
///
/// Returns `Ok(())` on successful completion or an I/O error.
///
/// # Behavior
///
/// - Receives byte counts from the channel
/// - Calculates total bytes, elapsed time, and transfer rate
/// - Updates display every 100ms (when timer is ready)
/// - Shows colored output: red for bytes, green for time, blue for rate
/// - Stops when receiving 0 bytes (EOF signal)
pub fn stats_loop(silent: bool, stats_rx: Receiver<usize>) -> Result<()> {
    let mut total_bytes = 0;
    let start = Instant::now();
    let mut timer = Timer::new();
    let mut stderr = io::stderr();

    while let Ok(num_bytes) = stats_rx.recv() {
        timer.update();
        let rate_per_second = num_bytes as f64 / timer.delta.as_secs_f64();
        total_bytes += num_bytes;
        if !silent && timer.ready {
            timer.ready = false;
            output_progress(
                &mut stderr,
                total_bytes,
                start.elapsed().as_secs().as_time(),
                rate_per_second,
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

/// Outputs formatted progress statistics to stderr with colored text.
///
/// # Arguments
///
/// * `stderr` - Standard error output handle.
/// * `bytes` - Total bytes transferred.
/// * `elapsed` - Elapsed time as formatted string (HH:MM:SS).
/// * `rate` - Transfer rate in bytes per second.
///
/// # Output Format
///
/// Displays: `{bytes} {elapsed} [{rate}b/s]` with colors:
/// - Red: bytes count
/// - Green: elapsed time
/// - Blue: transfer rate
fn output_progress(stderr: &mut Stderr, bytes: usize, elapsed: String, rate: f64) {
    let bytes = style::style(format!("{} ", bytes)).with(Color::Red);
    let elapsed = style::style(elapsed).with(Color::Green);
    let rate = style::style(format!(" [{:.0}b/s]", rate)).with(Color::Blue);
    let _ = execute!(
        stderr,
        cursor::MoveToColumn(0),
        Clear(ClearType::CurrentLine),
        PrintStyledContent(bytes),
        PrintStyledContent(elapsed),
        PrintStyledContent(rate)
    );
    let _ = stderr.flush();
}

/// Simple stats function (legacy).
///
/// # Arguments
///
/// * `silent` - If true, suppresses output.
/// * `num_read` - Number of bytes read in this iteration.
/// * `total_bytes` - Mutable reference to total bytes counter.
/// * `last` - If true, this is the final stats update.
///
/// # Note
///
/// This function is kept for compatibility but is not used in the current
/// multi-threaded implementation. Use [`stats_loop`] instead.
pub fn stats(silent: bool, num_read: usize, total_bytes: &mut usize, last: bool) {
    *total_bytes += num_read;

    if !silent {
        eprint!("\r{total_bytes}");
        if last {
            eprintln!();
        }
    }
}

#[cfg(test)]
mod test {
    use super::TimeOutput;

    #[test]
    fn test_time_format() {
        let pairs: Vec<(u64, &str)> = vec![
            (5, "0:00:05"),
            (60, "0:01:00"),
            (154, "0:02:34"),
            (3603, "1:00:03"),
        ];

        for (i, o) in pairs {
            assert_eq!(i.as_time().as_str(), o);
        }
    }
}
