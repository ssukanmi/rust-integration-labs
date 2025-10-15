//! Timer for controlling stats display update frequency.

use std::time::{Duration, Instant};

/// A timer that tracks elapsed time and controls update frequency.
///
/// The timer updates at regular intervals (100ms by default) to throttle
/// how often progress statistics are displayed to the terminal.
pub struct Timer {
    /// The last time the timer was updated
    pub last_instant: Instant,
    /// Time elapsed since last update
    pub delta: Duration,
    /// Update period (100ms)
    pub period: Duration,
    /// Countdown to next ready state
    pub count_down: Duration,
    /// Whether the timer is ready for the next update
    pub ready: bool,
}

impl Timer {
    /// Creates a new timer with a 100ms update period.
    ///
    /// The timer starts in the ready state.
    pub fn new() -> Self {
        let now = Instant::now();
        Self {
            last_instant: now,
            delta: Duration::default(),
            period: Duration::from_millis(100),
            count_down: Duration::default(),
            ready: true,
        }
    }

    /// Updates the timer and checks if it's ready for the next display update.
    ///
    /// Calculates the time delta since last update and decrements the countdown.
    /// When the countdown reaches zero, sets `ready` to `true` and resets the countdown.
    pub fn update(&mut self) {
        let now = Instant::now();
        self.delta = now - self.last_instant;
        self.last_instant = now;
        self.count_down = self.count_down.checked_sub(self.delta).unwrap_or_else(|| {
            self.ready = true;
            self.period
        });
    }
}
