use std::{collections::HashMap, io::Cursor};

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

use crate::{TrayEvent, TraySpy, Util};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IconEvent {
  HoverEnter,
  HoverLeave,
  HoverMove,
  LeftClick,
  RightClick,
  MiddleClick,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StableId {
  HandleUid(isize, u32),
  Guid(uuid::Uuid),
}

impl ToString for StableId {
  fn to_string(&self) -> String {
    match self {
      StableId::HandleUid(handle, uid) => format!("{}:{}", handle, uid),
      StableId::Guid(guid) => guid.to_string(),
    }
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SystrayIcon {
  pub stable_id: StableId,
  pub uid: Option<u32>,
  pub window_handle: Option<isize>,
  pub guid: Option<uuid::Uuid>,
  pub tooltip: String,
  pub icon: image::RgbaImage,
  pub callback: u32,
  pub version: u32,
}

impl SystrayIcon {
  /// Converts the icon to a PNG byte vector.
  pub fn to_png(&self) -> crate::Result<Vec<u8>> {
    let mut png_bytes: Vec<u8> = Vec::new();

    self
      .icon
      .write_to(&mut Cursor::new(&mut png_bytes), image::ImageFormat::Png)
      .map_err(|_| crate::Error::IconConversionFailed)?;

    Ok(png_bytes)
  }
}

#[derive(Debug)]
pub struct Systray {
  icons: HashMap<u32, SystrayIcon>,
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

  pub fn icons(&self) -> Vec<SystrayIcon> {
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
        tracing::info!("New icon added: {:?}", icon_data);
        // self.icons.insert(icon_data.uid, icon_data.clone());
      }
      TrayEvent::IconUpdate(icon_data) => {
        tracing::info!("Icon modified: {:?}", icon_data);
        // self.icons.insert(icon_data.uid, icon_data.clone());
      }
      TrayEvent::IconRemove(icon_data) => {
        tracing::info!("Icon removed: {:?}", icon_data);
        // self.icons.remove(icon_data.uid);
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

    let Some(window_handle) = icon.window_handle else {
      return Ok(());
    };

    // Checks whether the window associated with the given handle still
    // exists. If the window is invalid, removes the corresponding icon
    // from the collection.
    if !unsafe { IsWindow(HWND(window_handle as _)) }.as_bool() {
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
    if icon.version >= 3 {
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
    icon: &SystrayIcon,
    message: u32,
  ) -> crate::Result<()> {
    // // The wparam is the mouse position for version > 3 (with the low
    // and // high word being the x and y-coordinates respectively),
    // and the UID // for version <= 3.
    // let wparam = if icon.version > 3 {
    //   let cursor_pos = Util::cursor_position()?;
    //   Util::make_lparam(cursor_pos.0 as i16, cursor_pos.1 as i16) as u32
    // } else {
    //   icon.uid
    // };

    // // The high word for the lparam is the UID for version > 3, and 0
    // for // version <= 3. The low word is always the message.
    // let lparam = if icon.version > 3 {
    //   Util::make_lparam(message as i16, 0)
    // } else {
    //   Util::make_lparam(message as i16, icon.uid as i16)
    // };

    // unsafe {
    //   SendNotifyMessageW(
    //     HWND(icon.window_handle as _),
    //     icon.callback,
    //     WPARAM(wparam as _),
    //     LPARAM(lparam as _),
    //   )
    // }?;

    Ok(())
  }
}
