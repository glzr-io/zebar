use std::{future::Future, time::Duration};

use tokio::task::JoinHandle;

pub async fn set_interval<T, F>(delay_ms: u64, callback: T) -> JoinHandle<()>
where
  // T: Future + Send + 'static + Copy,
  T: (Fn() -> F) + Send + Sync + 'static,
  F: Future + Send,
{
  // The interval time alignment is decided at construction time.
  // For all calls to be evenly spaced, the interval must be constructed first.
  let mut interval = tokio::time::interval(Duration::from_millis(delay_ms));

  // The first tick happens without delay.
  // Whether to tick before the first do_something or after doesn't matter.
  interval.tick().await;
  callback().await;

  tokio::task::spawn(async move {
    loop {
      interval.tick().await;
      callback().await;
    }
  })
}
