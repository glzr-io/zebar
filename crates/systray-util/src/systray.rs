use std::{
  collections::HashMap,
  fmt::{self, Display},
  hash::{DefaultHasher, Hash, Hasher},
  io::Cursor,
  str::FromStr,
};

pub use image::ImageFormat;
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

#[derive(Clone, Eq, PartialEq)]
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

  /// Icon image.
  pub icon_image: Option<image::RgbaImage>,

  /// Hash of the icon image.
  ///
  /// Used to determine if the icon image has changed without having to
  /// compare the entire image.
  pub icon_image_hash: Option<String>,

  /// Application-defined message identifier.
  ///
  /// Used to send messages to the window that contains the icon.
  pub callback_message: Option<u32>,

  /// Version of the icon.
  pub version: Option<u32>,

  /// Whether the icon is visible in the system tray.
  ///
  /// This is determined by the `NIS_HIDDEN` flag in the icon's state.
  pub is_visible: bool,
}

// Debug implementation for `SystrayIcon`. Icon image is a large
// buffer, so we trim it in the debug output.
impl std::fmt::Debug for SystrayIcon {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("SystrayIcon")
      .field("stable_id", &self.stable_id)
      .field("uid", &self.uid)
      .field("window_handle", &self.window_handle)
      .field("guid", &self.guid)
      .field("tooltip", &self.tooltip)
      .field("icon_handle", &self.icon_handle)
      .field("icon_image", &self.icon_image.as_ref().map(|_| "..."))
      .field("callback_message", &self.callback_message)
      .field("version", &self.version)
      .field("is_visible", &self.is_visible)
      .finish()
  }
}

impl SystrayIcon {
  /// Converts the icon image to a byte vector of the given image format.
  pub fn to_image_format(
    &self,
    format: ImageFormat,
  ) -> crate::Result<Vec<u8>> {
    let mut bytes: Vec<u8> = Vec::new();

    self
      .icon_image
      .as_ref()
      .ok_or(crate::Error::InoperableIcon)?
      .write_to(&mut Cursor::new(&mut bytes), format)
      .map_err(|_| crate::Error::IconConversionFailed)?;

    Ok(bytes)
  }

  /// Computes a hash of the icon image.
  pub fn icon_image_hash(icon_image: &image::RgbaImage) -> String {
    let mut hasher = DefaultHasher::new();
    icon_image.as_raw().hash(&mut hasher);
    format!("{:x}", hasher.finish())
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
  _spy: TraySpy,
  event_rx: tokio::sync::mpsc::UnboundedReceiver<TrayEvent>,
}

impl Systray {
  /// Creates a new `Systray` instance.
  pub fn new() -> crate::Result<Self> {
    let (_spy, event_rx) = TraySpy::new()?;

    Ok(Systray {
      icons: HashMap::new(),
      _spy,
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

        let found_icon_id =
          self.find_icon(icon_data).map(|icon| icon.stable_id.clone());

        let found_icon = match found_icon_id {
          Some(id) => self.icons.get_mut(&id),
          None => None,
        };

        // Update the icon in-place if found.
        if let Some(found_icon) = found_icon {
          if let Some(uid) = icon_data.uid {
            found_icon.uid = Some(uid);
          }

          if let Some(window_handle) = icon_data.window_handle {
            found_icon.window_handle = Some(window_handle);
          }

          if let Some(guid) = icon_data.guid {
            found_icon.guid = Some(guid);
          }

          if let Some(tooltip) = &icon_data.tooltip {
            found_icon.tooltip = tooltip.clone();
          }

          if let Some(icon_handle) = icon_data.icon_handle {
            // Avoid re-reading the icon image if it's the same as the
            // existing icon.
            if found_icon.icon_handle != Some(icon_handle) {
              if let Ok(new_icon_image) = Util::icon_to_image(icon_handle)
              {
                found_icon.icon_handle = Some(icon_handle);
                found_icon.icon_image_hash =
                  Some(SystrayIcon::icon_image_hash(&new_icon_image));
                found_icon.icon_image = Some(new_icon_image);
              }
            }
          }

          if let Some(callback_message) = icon_data.callback_message {
            found_icon.callback_message = Some(callback_message);
          }

          if let Some(version) = icon_data.version {
            found_icon.version = Some(version);
          }

          found_icon.is_visible = icon_data.is_visible;

          Some(SystrayEvent::IconUpdate(found_icon.clone()))
        } else {
          // Icon doesn't exist yet, so add new icon. Skip icons that
          // cannot be identified.
          let stable_id = icon_data.guid.map(StableId::Guid).or({
            match (icon_data.window_handle, icon_data.uid) {
              (Some(handle), Some(uid)) => {
                Some(StableId::HandleUid(handle, uid))
              }
              _ => None,
            }
          })?;

          let icon_image = icon_data
            .icon_handle
            .and_then(|icon_handle| Util::icon_to_image(icon_handle).ok());

          let icon_image_hash =
            icon_image.as_ref().map(SystrayIcon::icon_image_hash);

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
            icon_image,
            icon_image_hash,
            callback_message: icon_data.callback_message,
            version: icon_data.version,
            is_visible: icon_data.is_visible,
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
    icon_id: &StableId,
    action: &SystrayIconAction,
  ) -> crate::Result<()> {
    tracing::info!("Sending icon action: {:?}", self.icons);
    let icon =
      self.icons.get(icon_id).ok_or(crate::Error::IconNotFound)?;

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
