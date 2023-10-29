pub async fn set_interval<T, F>(interval: std::time::Duration, do_something: T)
where
  T: (Fn() -> F) + Send + Sync + 'static,
  F: std::future::Future + Send,
{
  // The interval time alignment is decided at construction time.
  // For all calls to be evenly spaced, the interval must be constructed first.
  let mut interval = tokio::time::interval(interval);

  // The first tick happens without delay.
  // Whether to tick before the first do_something or after doesn't matter.
  interval.tick().await;
  do_something().await;

  tokio::task::spawn(async move {
    loop {
      interval.tick().await;
      do_something().await;
    }
  });
}
