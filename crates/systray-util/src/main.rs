use systray_util::Systray;

fn main() -> systray_util::Result<()> {
  tracing_subscriber::fmt().init();

  let mut systray = Systray::new()?;

  while let Some(event) = systray.events_blocking() {
    println!("Event: {:?}", event);
  }

  Ok(())
}
