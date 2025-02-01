use systray_util::Systray;

fn main() -> systray_util::Result<()> {
  tracing_subscriber::fmt().init();

  let mut systray = Systray::new()?;

  while let Some(event) = systray.changes() {
    println!("Event: {:?}", event);
  }

  Ok(())
}
