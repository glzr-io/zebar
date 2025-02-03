use std::collections::HashMap;

use windows::Win32::{
  Foundation::{HWND, LPARAM, WPARAM},
  UI::{
    Controls::{WM_MOUSEHOVER, WM_MOUSELEAVE},
    Shell::{NIN_POPUPCLOSE, NIN_POPUPOPEN, NIN_SELECT},
    WindowsAndMessaging::{
      IsWindow, SendNotifyMessageW, WM_CONTEXTMENU, WM_LBUTTONDOWN,
      WM_LBUTTONUP, WM_MBUTTONDOWN, WM_MBUTTONUP, WM_MOUSEMOVE,
      WM_RBUTTONDOWN, WM_RBUTTONUP,
    },
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
  icons: HashMap<u32, IconData>,
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

  pub fn icons(&self) -> Vec<IconData> {
    self.icons.values().cloned().collect()
  }

  pub async fn events(&mut self) -> Option<TrayEvent> {
    if let Some(event) = self.event_rx.recv().await {
      self.on_event(event)
    } else {
      None
    }
  }

  pub fn events_blocking(&mut self) -> Option<TrayEvent> {
    if let Some(event) = self.event_rx.blocking_recv() {
      self.on_event(event)
    } else {
      None
    }
  }

  fn on_event(&mut self, event: TrayEvent) -> Option<TrayEvent> {
    match &event {
      TrayEvent::IconAdd(icon_data) => {
        if let Some(uid) = icon_data.uid {
          tracing::info!(
            "New icon added: {} ({})",
            icon_data.tooltip.as_deref().unwrap_or(""),
            uid
          );
          self.icons.insert(uid, icon_data.clone());
        }
      }
      TrayEvent::IconUpdate(icon_data) => {
        if let Some(uid) = icon_data.uid {
          tracing::info!(
            "Icon modified: {} ({})",
            icon_data.tooltip.as_deref().unwrap_or(""),
            uid
          );
          if let Some(existing_icon) = self.icons.get_mut(&uid) {
            existing_icon.update(icon_data);
          } else {
            self.icons.insert(uid, icon_data.clone());
          }
        }
      }
      TrayEvent::IconRemove(uid) => {
        tracing::info!("Icon removed: {:#x}", uid);
        self.icons.remove(uid);
      }
    }

    Some(event)
  }

  pub fn send_icon_event(
    &mut self,
    icon_uid: u32,
    event: IconEvent,
  ) -> crate::Result<()> {
    let icon = self
      .icons
      .get(&icon_uid)
      .ok_or(crate::Error::IconNotFound)?;

    // Check if we have a valid window handle
    if let Some(window_handle) = icon.window_handle {
      // Checks whether the window associated with the given handle still
      // exists. If the window is invalid, removes the corresponding icon
      // from the collection.
      if !unsafe { IsWindow(HWND(window_handle as _)) }.as_bool() {
        self.icons.remove(&icon_uid);
        return Ok(());
      }
    } else {
      return Ok(());
    }

    let wm_messages = match event {
      IconEvent::LeftClick => vec![WM_LBUTTONDOWN, WM_LBUTTONUP],
      IconEvent::RightClick => vec![WM_RBUTTONDOWN, WM_RBUTTONUP],
      IconEvent::MiddleClick => {
        vec![WM_MBUTTONDOWN, WM_MBUTTONUP, WM_CONTEXTMENU]
      }
      IconEvent::HoverEnter => vec![WM_MOUSEHOVER],
      IconEvent::HoverLeave => vec![WM_MOUSELEAVE],
      IconEvent::HoverMove => vec![WM_MOUSEMOVE],
    };

    // TODO: Allow icon hwnd to gain focus for left/right/middle clicks.

    for wm_message in wm_messages {
      self.notify_icon(icon, wm_message)?;
    }

    // This is documented as version 4, but Explorer does this for version
    // 3 as well
    if icon.version.unwrap_or(0) >= 3 {
      let nin_message = match event {
        IconEvent::HoverEnter => NIN_POPUPOPEN,
        IconEvent::HoverLeave => NIN_POPUPCLOSE,
        IconEvent::LeftClick => NIN_SELECT,
        _ => return Ok(()),
      };

      self.notify_icon(icon, nin_message)?;
    }

    Ok(())
  }

  fn notify_icon(
    &self,
    icon: &IconData,
    message: u32,
  ) -> crate::Result<()> {
    // Early return if we don't have the required fields
    let (window_handle, uid, callback) =
      match (icon.window_handle, icon.uid, icon.callback) {
        (Some(wh), Some(u), Some(c)) => (wh, u, c),
        _ => return Ok(()),
      };

    // The wparam is the mouse position for version > 3 (with the low and
    // high word being the x and y-coordinates respectively), and the UID
    // for version <= 3.
    let wparam = if icon.version.unwrap_or(0) > 3 {
      let cursor_pos = Util::cursor_position()?;
      Util::make_lparam(cursor_pos.0 as i16, cursor_pos.1 as i16) as u32
    } else {
      uid
    };

    // The high word for the lparam is the UID for version > 3, and 0 for
    // version <= 3. The low word is always the message.
    let lparam = if icon.version.unwrap_or(0) > 3 {
      Util::make_lparam(message as i16, 0)
    } else {
      Util::make_lparam(message as i16, uid as i16)
    };

    unsafe {
      SendNotifyMessageW(
        HWND(window_handle as _),
        callback,
        WPARAM(wparam as _),
        LPARAM(lparam as _),
      )
    }?;

    Ok(())
  }
}
