#[cfg(target_os = "windows")]
use systray_util::{Systray, SystrayEvent};

#[cfg(target_os = "windows")]
fn main() -> systray_util::Result<()> {
  let mut systray = Systray::new()?;

  while let Some(event) = systray.events_blocking() {
    match event {
      SystrayEvent::IconAdd(icon) => {
        println!("Tray icon added: {:?}", icon);
      }
      SystrayEvent::IconUpdate(icon) => {
        println!("Tray icon updated: {:?}", icon);
      }
      SystrayEvent::IconRemove(id) => {
        println!("Tray icon removed: {:?}", id);
      }
    }
  }

  Ok(())
}

// systray-util is Windows-only. This stub keeps the example target
// compilable elsewhere,
#[cfg(not(target_os = "windows"))]
fn main() {}
