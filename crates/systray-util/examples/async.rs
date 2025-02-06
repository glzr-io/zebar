use systray_util::{Systray, SystrayEvent};

#[tokio::main]
async fn main() -> systray_util::Result<()> {
  let mut systray = Systray::new()?;

  while let Some(event) = systray.events().await {
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
