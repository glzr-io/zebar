use systray_util::run;

fn main() -> systray_util::Result<()> {
  tracing_subscriber::fmt().init();

  run()
}
