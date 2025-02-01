use std::collections::HashMap;

use windows::Win32::{
  Foundation::{HWND, LPARAM, POINT, WPARAM},
  UI::{
    Shell::NIN_SELECT,
    WindowsAndMessaging::{
      GetCursorPos, IsWindow, SendNotifyMessageW, WM_LBUTTONUP,
    },
  },
};

use crate::{IconData, TrayEvent, TraySpy, Util};

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

  pub fn send_left_click(&mut self, icon_uid: u32) -> crate::Result<()> {
    let icon = self
      .icons
      .get(&icon_uid)
      .ok_or(crate::Error::IconNotFound)?;

    if self.remove_if_invalid(&icon) {
      return Ok(());
    }

    let mouse = Self::cursor_param()?;
    self.send_notify_message(icon, WM_LBUTTONUP, mouse);

    // This is documented as version 4, but Explorer does this for version
    // 3 as well
    if icon.version >= 3 {
      self.send_notify_message(icon, NIN_SELECT, mouse);
    }

    Ok(())
  }

  fn cursor_param() -> crate::Result<LPARAM> {
    let mut point = POINT { x: 0, y: 0 };
    unsafe { GetCursorPos(&mut point) }?;
    Ok(Util::make_lparam(point.x as u16, point.y as u16))
  }

  fn send_notify_message(
    &self,
    icon: &IconData,
    message: u32,
    mouse: u32,
  ) -> crate::Result<()> {
    let wparam = self.get_message_wparam(icon, mouse);
    let hiword = self.get_message_hiword(icon);
    let lparam = message | (hiword << 16);

    unsafe {
      SendNotifyMessageW(
        HWND(icon.window_handle as _),
        icon.callback,
        WPARAM(wparam as _),
        LPARAM(lparam as _),
      )
    };
    Ok(())
  }

  /// Gets the high word for the message based on icon version
  ///
  /// # Arguments
  /// * `icon` - The icon data containing version information
  ///
  /// # Returns
  /// * The high word value (either the UID for version > 3, or 0)
  fn get_message_hiword(&self, icon: &IconData) -> u32 {
    if icon.version > 3 {
      icon.uid
    } else {
      0
    }
  }

  /// Gets the wparam for the message based on icon version
  ///
  /// # Arguments
  /// * `icon` - The icon data containing version information
  /// * `mouse` - Mouse event data
  ///
  /// # Returns
  /// * The wparam value (either the mouse data for version > 3, or the
  ///   UID)
  fn get_message_wparam(&self, icon: &IconData, mouse: u32) -> u32 {
    if icon.version > 3 {
      mouse
    } else {
      icon.uid
    }
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
