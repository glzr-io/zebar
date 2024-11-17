use std::{
  thread,
  time::{Duration, Instant},
};

/// A synchronous timer for running intervals at a fixed rate.
///
/// The timer sleeps the thread until the next tick.
pub struct SyncInterval {
  interval: Duration,
  next_tick: Instant,
}

impl SyncInterval {
  pub fn new(interval_ms: u64) -> Self {
    Self {
      interval: Duration::from_millis(interval_ms),
      next_tick: Instant::now(),
    }
  }

  /// Sleeps until the next tick if needed, then updates the next tick
  /// time.
  pub fn tick(&mut self) {
    // Sleep until next tick if needed.
    if let Some(wait_duration) =
      self.next_tick.checked_duration_since(Instant::now())
    {
      thread::sleep(wait_duration);
    }

    // Calculate next tick time.
    self.next_tick += self.interval;
  }
}

/// An asynchronous timer for running intervals at a fixed rate.
///
/// The timer sleeps the Tokio task until the next tick.
pub struct AsyncInterval {
  interval: tokio::time::Interval,
}

impl AsyncInterval {
  pub fn new(interval_ms: u64) -> Self {
    let mut interval =
      tokio::time::interval(Duration::from_millis(interval_ms));

    // Skip missed ticks when the interval runs. This prevents a burst
    // of backlogged ticks after a delay.
    interval
      .set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

    Self { interval }
  }

  pub async fn tick(&mut self) {
    self.interval.tick().await;
  }
}
