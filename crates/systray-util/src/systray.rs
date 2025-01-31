use std::collections::HashMap;

use crate::{IconData, TrayEvent, TraySpy};

pub struct Systray {
  pub icons: HashMap<u32, IconData>,
  event_rx: tokio::sync::mpsc::UnboundedReceiver<TrayEvent>,
}

impl Systray {
  pub fn new() -> crate::Result<Self> {
    let (_spy, event_rx) = TraySpy::new()?;
    Ok(Systray {
      icons: HashMap::new(),
      event_rx,
    })
  }

  pub fn changes(&mut self) -> Option<TrayEvent> {
    if let Some(event) = self.event_rx.blocking_recv() {
      match &event {
        TrayEvent::IconAdd(icon_data) => {
          tracing::info!(
            "New icon added: {} ({})",
            icon_data.tooltip,
            icon_data.uid
          );
          println!("New icon added: {:?}", icon_data);
          self.icons.insert(icon_data.uid, icon_data.clone());
        }
        TrayEvent::IconUpdate(icon_data) => {
          tracing::info!(
            "Icon modified: {} ({})",
            icon_data.tooltip,
            icon_data.uid
          );
          println!("Icon modified: {:?}", icon_data);
          self.icons.insert(icon_data.uid, icon_data.clone());
        }
        TrayEvent::IconRemove(uid) => {
          tracing::info!("Icon removed: {:#x}", uid);
          self.icons.remove(uid);
        }
      }
      Some(event)
    } else {
      None
    }
  }
}
