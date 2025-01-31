use systray_util::Systray;

fn main() -> systray_util::Result<()> {
  tracing_subscriber::fmt().init();

  let systray = Systray::new()?;

  Ok(())
}
