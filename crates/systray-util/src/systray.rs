use std::{
  collections::HashMap,
  fmt::{self, Display},
  io::Cursor,
  str::FromStr,
};

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

use crate::{IconEventData, TrayEvent, TraySpy, Util};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IconEvent {
  HoverEnter,
  HoverLeave,
  HoverMove,
  LeftClick,
  RightClick,
  MiddleClick,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum StableId {
  HandleUid(isize, u32),
  Guid(uuid::Uuid),
}

impl Display for StableId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      StableId::HandleUid(handle, uid) => write!(f, "{}:{}", handle, uid),
      StableId::Guid(guid) => write!(f, "{}", guid),
    }
  }
}

impl FromStr for StableId {
  type Err = crate::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    // Try parsing as handle and uid (format: "handle:uid").
    if let Some((handle_str, uid_str)) = s.split_once(':') {
      return Ok(StableId::HandleUid(
        handle_str
          .parse()
          .map_err(|_| crate::Error::InvalidIconId)?,
        uid_str.parse().map_err(|_| crate::Error::InvalidIconId)?,
      ));
    }

    // Try parsing as a guid.
    if let Ok(guid) = uuid::Uuid::parse_str(s) {
      return Ok(StableId::Guid(guid));
    }

    Err(crate::Error::InvalidIconId)
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SystrayIcon {
  /// Identifier for the icon. Will not change for the lifetime of the
  /// icon.
  ///
  /// The Windows shell uses either a (window handle + uid) or its guid
  /// to identify which icon to operate on.
  ///
  /// Read more: https://learn.microsoft.com/en-us/windows/win32/api/shellapi/ns-shellapi-notifyicondataw
  pub stable_id: StableId,

  /// Application-defined identifier for the icon, used in combination
  /// with the window handle.
  ///
  /// The uid only has to be unique for the window handle. Multiple
  /// icons (across different window handles) can have the same uid.
  pub uid: Option<u32>,

  /// Handle to the window that contains the icon. Used in combination
  /// with a uid.
  ///
  /// Note that multiple icons can have the same window handle.
  pub window_handle: Option<isize>,

  /// GUID for the icon.
  ///
  /// Used as an alternate way to identify the icon (versus its window
  /// handle and uid).
  pub guid: Option<uuid::Uuid>,

  /// Tooltip to show for the icon on hover.
  pub tooltip: String,

  /// Handle to the icon bitmap.
  pub icon_handle: Option<isize>,

  /// Application-defined message identifier.
  ///
  /// Used to send messages to the window that contains the icon.
  pub callback_message: Option<u32>,

  /// Version of the icon.
  pub version: Option<u32>,
}

impl SystrayIcon {
  /// Converts the icon to a `RgbaImage`.
  pub fn to_image(&self) -> crate::Result<image::RgbaImage> {
    let icon_handle =
      self.icon_handle.ok_or(crate::Error::IconNotFound)?;

    Util::icon_to_image(icon_handle)
  }

  /// Converts the icon to a PNG byte vector.
  pub fn to_png(&self) -> crate::Result<Vec<u8>> {
    let mut png_bytes: Vec<u8> = Vec::new();

    self
      .to_image()?
      .write_to(&mut Cursor::new(&mut png_bytes), image::ImageFormat::Png)
      .map_err(|_| crate::Error::IconConversionFailed)?;

    Ok(png_bytes)
  }
}

#[derive(Debug)]
pub struct Systray {
  icons: HashMap<StableId, SystrayIcon>,
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

  pub fn icon_by_handle(
    &self,
    handle: isize,
    uid: u32,
  ) -> Option<&SystrayIcon> {
    self.icons.get(&StableId::HandleUid(handle, uid))
  }

  pub fn icon_by_guid(&self, guid: uuid::Uuid) -> Option<&SystrayIcon> {
    self.icons.get(&StableId::Guid(guid))
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
      TrayEvent::IconAdd(icon_data) | TrayEvent::IconUpdate(icon_data) => {
        tracing::info!("Icon modified or added: {:?}", icon_data);

        let found_icon = self.find_icon(icon_data);

        if let Some(found_icon) = found_icon {
          // Update existing icon with new data.
          let updated_icon = SystrayIcon {
            stable_id: found_icon.stable_id.clone(),
            uid: icon_data.uid.or(found_icon.uid),
            window_handle: icon_data
              .window_handle
              .or(found_icon.window_handle),
            guid: icon_data.guid.or(found_icon.guid),
            tooltip: icon_data
              .tooltip
              .clone()
              .unwrap_or(found_icon.tooltip.clone()),
            icon_handle: icon_data.icon_handle.or(found_icon.icon_handle),
            callback_message: icon_data
              .callback_message
              .or(found_icon.callback_message),
            version: icon_data.version.or(found_icon.version),
          };

          self
            .icons
            .insert(found_icon.stable_id.clone(), updated_icon);
        } else {
          // Icon doesn't exist yet, so add new icon.
          let stable_id = if let Some(guid) = icon_data.guid {
            StableId::Guid(guid)
          } else if let (Some(handle), Some(uid)) =
            (icon_data.window_handle, icon_data.uid)
          {
            StableId::HandleUid(handle, uid)
          } else {
            return Some(event);
          };

          let icon = SystrayIcon {
            stable_id,
            uid: icon_data.uid,
            window_handle: icon_data.window_handle,
            guid: icon_data.guid,
            tooltip: icon_data
              .tooltip
              .clone()
              .unwrap_or_else(|| "".to_string()),
            icon_handle: icon_data.icon_handle,
            callback_message: icon_data.callback_message,
            version: icon_data.version,
          };

          self.icons.insert(icon.stable_id.clone(), icon);
        }
      }
      TrayEvent::IconRemove(icon_data) => {
        tracing::info!("Icon removed: {:?}", icon_data);

        let icon_id =
          self.find_icon(icon_data).map(|icon| icon.stable_id.clone());

        if let Some(icon_id) = icon_id {
          self.icons.remove(&icon_id);
        }
      }
    }

    Some(event)
  }

  fn find_icon(&self, icon_data: &IconEventData) -> Option<&SystrayIcon> {
    icon_data
      .guid
      .and_then(|guid| self.icon_by_guid(guid))
      .or_else(|| match (icon_data.window_handle, icon_data.uid) {
        (Some(handle), Some(uid)) => self.icon_by_handle(handle, uid),
        _ => None,
      })
  }

  pub fn send_icon_event(
    &mut self,
    icon_id: StableId,
    event: IconEvent,
  ) -> crate::Result<()> {
    let icon =
      self.icons.get(&icon_id).ok_or(crate::Error::IconNotFound)?;

    // Early return if we don't have the required fields.
    let window_handle =
      icon.window_handle.ok_or(crate::Error::InoperableIcon)?;
    let uid = icon.uid.ok_or(crate::Error::InoperableIcon)?;
    let callback =
      icon.callback_message.ok_or(crate::Error::InoperableIcon)?;

    // Checks whether the window associated with the given handle still
    // exists. If the window is invalid, removes the corresponding icon
    // from the collection.
    if !unsafe { IsWindow(HWND(window_handle as _)) }.as_bool() {
      return Err(crate::Error::InoperableIcon);
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
      Self::notify_icon(
        window_handle,
        callback,
        uid,
        icon.version,
        wm_message,
      )?;
    }

    // This is documented as version 4, but Explorer does this for version
    // 3 as well
    if icon.version.is_some_and(|version| version >= 3) {
      let nin_message = match event {
        IconEvent::HoverEnter => NIN_POPUPOPEN,
        IconEvent::HoverLeave => NIN_POPUPCLOSE,
        IconEvent::LeftClick => NIN_SELECT,
        _ => return Ok(()),
      };

      Self::notify_icon(
        window_handle,
        callback,
        uid,
        icon.version,
        nin_message,
      )?;
    }

    Ok(())
  }

  fn notify_icon(
    window_handle: isize,
    callback: u32,
    uid: u32,
    version: Option<u32>,
    message: u32,
  ) -> crate::Result<()> {
    // The wparam is the mouse position for version > 3 (with the low and
    // high word being the x and y-coordinates respectively), and the UID
    // for version <= 3.
    let wparam = if version.is_some_and(|version| version > 3) {
      let cursor_pos = Util::cursor_position()?;
      Util::pack_i32(cursor_pos.0 as i16, cursor_pos.1 as i16) as u32
    } else {
      uid
    };

    // The high word for the lparam is the UID for version > 3, and 0 for
    // version <= 3. The low word is always the message.
    let lparam = if version.is_some_and(|version| version > 3) {
      Util::pack_i32(message as i16, 0)
    } else {
      Util::pack_i32(message as i16, uid as i16)
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
