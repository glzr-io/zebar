use std::collections::HashMap;

use windows::Win32::{
  Foundation::{HWND, LPARAM, WPARAM},
  UI::{
    Shell::NIN_SELECT,
    WindowsAndMessaging::{IsWindow, SendNotifyMessageW, WM_LBUTTONUP},
  },
};

use crate::{IconData, TrayEvent, TraySpy, Util};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IconEvent {
  HoverEnter,
  HoverLeave,
  HoverMove,
  LeftClick,
  RightClick,
  MiddleClick,
}

#[derive(Debug)]
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
          self.icons.insert(icon_data.uid, icon_data.clone());
        }
        TrayEvent::IconUpdate(icon_data) => {
          tracing::info!(
            "Icon modified: {} ({})",
            icon_data.tooltip,
            icon_data.uid
          );
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

  pub fn send_left_click(&mut self, icon_uid: u32) -> crate::Result<()> {
    let icon = self
      .icons
      .get(&icon_uid)
      .ok_or(crate::Error::IconNotFound)?;

    if !unsafe { IsWindow(HWND(icon.window_handle as _)) }.as_bool() {
      self.icons.remove(&icon_uid);
      return Ok(());
    }

    self.send_icon_message(icon, WM_LBUTTONUP);

    // This is documented as version 4, but Explorer does this for version
    // 3 as well
    if icon.version >= 3 {
      self.send_icon_message(icon, NIN_SELECT);
    }

    Ok(())
  }

  fn send_icon_message(
    &self,
    icon: &IconData,
    message: u32,
  ) -> crate::Result<()> {
    // The wparam is the mouse position for version > 3 (with the low and
    // high word being the x and y-coordinates respectively), and the UID
    // for version <= 3.
    let wparam = if icon.version > 3 {
      let cursor_pos = Util::cursor_position()?;
      Util::make_lparam(cursor_pos.0 as i16, cursor_pos.1 as i16) as u32
    } else {
      icon.uid
    };

    // The high word for the lparam is the UID for version > 3, and 0 for
    // version <= 3. The low word is always the message.
    let lparam = if icon.version > 3 {
      Util::make_lparam(message as i16, 0)
    } else {
      Util::make_lparam(message as i16, icon.uid as i16)
    };

    unsafe {
      SendNotifyMessageW(
        HWND(icon.window_handle as _),
        icon.callback,
        WPARAM(wparam as _),
        LPARAM(lparam as _),
      )
    }?;

    Ok(())
  }

  /// Checks whether the window associated with the given handle still
  /// exists. If the window is invalid, removes the corresponding icon
  /// from the collection.
  ///
  /// Returns `true` if the icon was removed, `false` otherwise.
  fn remove_if_invalid(&mut self, icon: &IconData) -> bool {
    if unsafe { IsWindow(HWND(icon.window_handle as _)) }.as_bool() {
      return false;
    }

    self.icons.remove(&icon.uid);
    return true;
  }
}
