use std::collections::HashMap;

use crate::{TrayEvent, TraySpy};

pub fn run() -> crate::Result<()> {
  let (_spy, mut event_rx) = TraySpy::new()?;

  let mut icons = HashMap::new();

  while let Some(event) = event_rx.blocking_recv() {
    match event {
      TrayEvent::IconAdd(icon_data) => {
        tracing::info!(
          "New icon added: {} ({})",
          icon_data.tooltip,
          icon_data.uid
        );

        icons.insert(icon_data.uid, icon_data);
      }
      TrayEvent::IconUpdate(icon_data) => {
        tracing::info!(
          "Icon modified: {} ({})",
          icon_data.tooltip,
          icon_data.uid
        );

        icons.insert(icon_data.uid, icon_data);
      }
      TrayEvent::IconRemove(uid) => {
        tracing::info!("Icon removed: {:#x}", uid);

        icons.remove(&uid);
      }
    }
  }

  Ok(())
}
