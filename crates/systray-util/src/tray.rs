use crate::{TrayEvent, TraySpy};

pub fn run() -> crate::Result<()> {
  let (_spy, mut event_rx) = TraySpy::new()?;

  while let Some(event) = event_rx.blocking_recv() {
    match event {
      TrayEvent::IconAdd(icon_data) => {
        println!(
          "New icon added: {} ({})",
          icon_data.tooltip, icon_data.uid
        );
      }
      TrayEvent::IconUpdate(icon_data) => {
        println!(
          "Icon modified: {} ({})",
          icon_data.tooltip, icon_data.uid
        );
      }
      TrayEvent::IconRemove(uid) => {
        println!("Icon removed: {:#x}", uid);
      }
    }
  }

  Ok(())
}
