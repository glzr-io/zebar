use std::time::{Duration, Instant};

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
  pub fn tick(&mut self) -> crossbeam::channel::Receiver<Instant> {
    if let Some(wait_duration) =
      self.next_tick.checked_duration_since(Instant::now())
    {
      let timer = crossbeam::channel::after(wait_duration);
      self.next_tick += self.interval;
      timer
    } else {
      while self.next_tick <= Instant::now() {
        self.next_tick += self.interval;
      }
      crossbeam::channel::after(self.next_tick - Instant::now())
    }
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
