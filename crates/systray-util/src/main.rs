use systray_util::Systray;

fn main() -> systray_util::Result<()> {
  tracing_subscriber::fmt().init();

  let mut systray = Systray::new()?;

  while let Some(_) = systray.events_blocking() {
    println!("Event emitted");
  }

  Ok(())
}
