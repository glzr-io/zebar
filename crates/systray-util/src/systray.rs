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
      AllowSetForegroundWindow, GetWindowThreadProcessId, IsWindow,
      SendNotifyMessageW, WM_CONTEXTMENU, WM_LBUTTONDOWN, WM_LBUTTONUP,
      WM_MBUTTONDOWN, WM_MBUTTONUP, WM_MOUSEMOVE, WM_RBUTTONDOWN,
      WM_RBUTTONUP,
    },
  },
};

use crate::{IconEventData, TrayEvent, TraySpy, Util};

/// Identifier for a systray icon.
///
/// A systray icon is either identified by a (window handle + uid) or
/// its guid. Since a systray icon can be updated to also include a
/// guid or window handle/uid later on, a stable ID is useful for
/// consistently identifying an icon.
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

/// Events that can be emitted by `Systray`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SystrayEvent {
  IconAdd(SystrayIcon),
  IconUpdate(SystrayIcon),
  IconRemove(StableId),
}

/// Actions that can be performed on a `SystrayIcon`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SystrayIconAction {
  HoverEnter,
  HoverLeave,
  HoverMove,
  LeftClick,
  RightClick,
  MiddleClick,
}

/// A system tray manager.
///
/// Manages a collection of `SystrayIcon`s and allows sending actions
/// to them.
#[derive(Debug)]
pub struct Systray {
  icons: HashMap<StableId, SystrayIcon>,
  event_rx: tokio::sync::mpsc::UnboundedReceiver<TrayEvent>,
}

impl Systray {
  /// Creates a new `Systray` instance.
  pub fn new() -> crate::Result<Self> {
    let (_spy, event_rx) = TraySpy::new()?;

    Ok(Systray {
      icons: HashMap::new(),
      event_rx,
    })
  }

  /// Returns all icons managed by the `Systray`.
  pub fn icons(&self) -> Vec<SystrayIcon> {
    self.icons.values().cloned().collect()
  }

  /// Returns the icon with the given handle and uid.
  pub fn icon_by_handle(
    &self,
    handle: isize,
    uid: u32,
  ) -> Option<&SystrayIcon> {
    self.icons.get(&StableId::HandleUid(handle, uid))
  }

  /// Returns the icon with the given guid.
  pub fn icon_by_guid(&self, guid: uuid::Uuid) -> Option<&SystrayIcon> {
    self.icons.get(&StableId::Guid(guid))
  }

  /// Returns the icon with the given stable ID.
  pub fn icon_by_id(&self, id: StableId) -> Option<&SystrayIcon> {
    self.icons.get(&id)
  }

  /// Returns the next event from the `Systray`.
  pub async fn events(&mut self) -> Option<SystrayEvent> {
    while let Some(event) = self.event_rx.recv().await {
      if let Some(event) = self.on_event(event) {
        return Some(event);
      }
    }

    None
  }

  /// Returns the next event from the `Systray` (synchronously).
  pub fn events_blocking(&mut self) -> Option<SystrayEvent> {
    while let Some(event) = self.event_rx.blocking_recv() {
      if let Some(event) = self.on_event(event) {
        return Some(event);
      }
    }

    None
  }

  /// Handles an event from the `Systray`.
  ///
  /// Returns `None` if the event should be ignored (e.g. if an icon that
  /// doesn't exist was removed).
  fn on_event(&mut self, event: TrayEvent) -> Option<SystrayEvent> {
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
            .insert(found_icon.stable_id.clone(), updated_icon.clone());

          Some(SystrayEvent::IconUpdate(updated_icon))
        } else {
          // Icon doesn't exist yet, so add new icon.
          let stable_id = if let Some(guid) = icon_data.guid {
            StableId::Guid(guid)
          } else if let (Some(handle), Some(uid)) =
            (icon_data.window_handle, icon_data.uid)
          {
            StableId::HandleUid(handle, uid)
          } else {
            return None;
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

          self.icons.insert(icon.stable_id.clone(), icon.clone());
          Some(SystrayEvent::IconAdd(icon))
        }
      }
      TrayEvent::IconRemove(icon_data) => {
        tracing::info!("Icon removed: {:?}", icon_data);

        let icon_id =
          self.find_icon(icon_data).map(|icon| icon.stable_id.clone());

        if let Some(icon_id) = icon_id {
          self.icons.remove(&icon_id);
          Some(SystrayEvent::IconRemove(icon_id))
        } else {
          None
        }
      }
    }
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

  /// Sends an action to the systray icon.
  pub fn send_action(
    &mut self,
    icon_id: StableId,
    action: SystrayIconAction,
  ) -> crate::Result<()> {
    tracing::info!("Sending icon action: {:?}", self.icons);
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

    let is_mouse_click = matches!(
      action,
      SystrayIconAction::LeftClick
        | SystrayIconAction::RightClick
        | SystrayIconAction::MiddleClick
    );

    // For mouse clicks, there is often a menu that appears after the
    // click. Allow the notify icon to gain focus so that the menu can be
    // dismissed after clicking outside.
    if is_mouse_click {
      let mut proc_id = u32::default();
      unsafe {
        GetWindowThreadProcessId(
          HWND(window_handle as _),
          Some(&mut proc_id),
        )
      };

      let _ = unsafe { AllowSetForegroundWindow(proc_id) };
    }

    let wm_messages = match action {
      SystrayIconAction::LeftClick => vec![WM_LBUTTONDOWN, WM_LBUTTONUP],
      SystrayIconAction::RightClick => {
        vec![WM_RBUTTONDOWN, WM_RBUTTONUP]
      }
      SystrayIconAction::MiddleClick => {
        vec![WM_MBUTTONDOWN, WM_MBUTTONUP]
      }
      SystrayIconAction::HoverEnter => vec![WM_MOUSEHOVER],
      SystrayIconAction::HoverLeave => vec![WM_MOUSELEAVE],
      SystrayIconAction::HoverMove => vec![WM_MOUSEMOVE],
    };

    for wm_message in wm_messages {
      Self::notify_icon(
        window_handle,
        callback,
        uid,
        icon.version,
        wm_message,
      )?;
    }

    // Additional messages are sent for version 4 and above. Explorer sends
    // these for version 3 as well though, so we do the same.
    // Ref: https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-shell_notifyicona#remarks
    if icon.version.is_some_and(|version| version >= 3) {
      let v3_message = match action {
        SystrayIconAction::HoverEnter => NIN_POPUPOPEN,
        SystrayIconAction::HoverLeave => NIN_POPUPCLOSE,
        SystrayIconAction::LeftClick => NIN_SELECT,
        SystrayIconAction::RightClick => WM_CONTEXTMENU,
        _ => return Ok(()),
      };

      Self::notify_icon(
        window_handle,
        callback,
        uid,
        icon.version,
        v3_message,
      )?;
    }

    Ok(())
  }

  /// Sends a message to the systray icon window.
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
      Util::pack_i32(message as i16, uid as i16)
    } else {
      Util::pack_i32(message as i16, 0)
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
