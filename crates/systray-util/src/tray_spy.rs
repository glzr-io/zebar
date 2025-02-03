use std::{sync::OnceLock, thread::JoinHandle};

use image::RgbaImage;
use tokio::sync::mpsc;
use windows::Win32::{
  Foundation::{HWND, LPARAM, LRESULT, WPARAM},
  System::DataExchange::COPYDATASTRUCT,
  UI::{
    Shell::{
      NIF_GUID, NIF_ICON, NIF_MESSAGE, NIF_TIP, NIM_ADD, NIM_DELETE,
      NIM_MODIFY, NIM_SETVERSION, NOTIFYICONDATAW_0,
      NOTIFY_ICON_DATA_FLAGS, NOTIFY_ICON_INFOTIP_FLAGS,
      NOTIFY_ICON_MESSAGE, NOTIFY_ICON_STATE,
    },
    WindowsAndMessaging::{
      DefWindowProcW, PostMessageW, RegisterWindowMessageW, SendMessageW,
      SendNotifyMessageW, SetTimer, SetWindowPos, HWND_BROADCAST,
      HWND_TOPMOST, SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOSIZE,
      WM_ACTIVATEAPP, WM_COMMAND, WM_COPYDATA, WM_TIMER, WM_USER,
    },
  },
};
use windows_core::w;

use crate::Util;

/// Global instance of sender for tray events.
///
/// For use with window procedure.
static TRAY_EVENT_TX: OnceLock<mpsc::UnboundedSender<TrayEvent>> =
  OnceLock::new();

/// Tray message sent to `Shell_TrayWnd` and intercepted by our spy window.
#[repr(C)]
struct ShellTrayMessage {
  magic_number: i32,
  message_type: u32,
  icon_data: NotifyIconData,
  version: u32,
}

/// Contains the data for a system tray icon.
///
/// When `Shell_NotifyIcon` sends its message to `Shell_Traywnd`, it
/// actually uses a 32-bit handle for the hwnd. This makes it slightly
/// different than the `windows` crate's `NOTIFYICONDATAW` type.
#[repr(C)]
#[derive(Clone, Copy)]
struct NotifyIconData {
  callback_size: u32,
  window_handle: u32,
  uid: u32,
  flags: NOTIFY_ICON_DATA_FLAGS,
  callback_message: u32,
  icon: u32,
  tooltip: [u16; 128],
  state: NOTIFY_ICON_STATE,
  state_mask: NOTIFY_ICON_STATE,
  size_info: [u16; 256],
  anonymous: NOTIFYICONDATAW_0,
  info_title: [u16; 64],
  info_flags: NOTIFY_ICON_INFOTIP_FLAGS,
  guid_item: windows_core::GUID,
  balloon_icon: u32,
}

#[repr(C)]
#[derive(Debug)]
struct WINNOTIFYICONIDENTIFIER {
  dwMagic: i32,
  dwMessage: i32,
  cbSize: i32,
  dwPadding: i32,
  hWnd: u32,
  uID: u32,
  guidItem: windows_core::GUID,
}

impl ShellTrayMessage {
  fn tray_event(&self) -> Option<TrayEvent> {
    match NOTIFY_ICON_MESSAGE(self.message_type) {
      NIM_ADD => Some(TrayEvent::IconAdd(self.icon_data())),
      NIM_MODIFY | NIM_SETVERSION => {
        Some(TrayEvent::IconUpdate(self.icon_data()))
      }
      NIM_DELETE => Some(TrayEvent::IconRemove(self.icon_data())),
      _ => None,
    }
  }

  fn icon_data(&self) -> IconEventData {
    let icon = if self.icon_data.flags.0 & NIF_ICON.0 != 0 {
      Util::icon_to_image(self.icon_data.icon).ok()
    } else {
      None
    };

    let guid = if self.icon_data.flags.0 & NIF_GUID.0 != 0 {
      Some(uuid::Uuid::from_u128(self.icon_data.guid_item.to_u128()))
    } else {
      None
    };

    let tooltip = if self.icon_data.flags.0 & NIF_TIP.0 != 0 {
      let tooltip_str = String::from_utf16_lossy(&self.icon_data.tooltip)
        .replace(['\0', '\r'], "")
        .to_string();

      (!tooltip_str.is_empty()).then_some(tooltip_str)
    } else {
      None
    };

    let (window_handle, uid) = if self.icon_data.window_handle != 0 {
      (
        Some(self.icon_data.window_handle as isize),
        Some(self.icon_data.uid),
      )
    } else {
      (None, None)
    };

    let callback = if self.icon_data.flags.0 & NIF_MESSAGE.0 != 0 {
      Some(self.icon_data.callback_message)
    } else {
      None
    };

    let version = if unsafe { self.icon_data.anonymous.uVersion } > 0
      && unsafe { self.icon_data.anonymous.uVersion } <= 4
    {
      Some(unsafe { self.icon_data.anonymous.uVersion })
    } else {
      None
    };

    IconEventData {
      uid,
      window_handle,
      guid,
      tooltip,
      icon,
      callback,
      version,
    }
  }
}

/// Events emitted by the spy window.
#[derive(Debug)]
pub enum TrayEvent {
  IconAdd(IconEventData),
  IconUpdate(IconEventData),
  IconRemove(IconEventData),
}

#[derive(Debug, Clone, PartialEq)]
pub struct IconEventData {
  pub uid: Option<u32>,
  pub window_handle: Option<isize>,
  pub guid: Option<uuid::Uuid>,
  pub tooltip: Option<String>,
  pub icon: Option<RgbaImage>,
  pub callback: Option<u32>,
  pub version: Option<u32>,
}

/// A window that spies on system tray icon messages and broadcasts events.
pub(crate) struct TraySpy {
  window_thread: Option<JoinHandle<()>>,
}

impl TraySpy {
  /// Creates a new `TraySpy` instance.
  pub fn new() -> crate::Result<(Self, mpsc::UnboundedReceiver<TrayEvent>)>
  {
    let (event_tx, event_rx) = mpsc::unbounded_channel();

    // Add the sender for tray events to global state.
    TRAY_EVENT_TX
      .set(event_tx)
      .expect("Tray event sender already set.");

    let spy = Self {
      window_thread: Some(Self::spawn()?),
    };

    Ok((spy, event_rx))
  }

  /// Starts the spy window in its own thread.
  ///
  /// Returns a thread handle for the spy window.
  fn spawn() -> crate::Result<std::thread::JoinHandle<()>> {
    let handle = std::thread::spawn(move || {
      if let Err(err) = Self::run() {
        tracing::error!("SpyWindow error: {:?}", err);
      }
    });

    Ok(handle)
  }

  /// Creates the spy window and runs its message loop.
  fn run() -> crate::Result<()> {
    let window = Util::create_message_window(
      "Shell_TrayWnd",
      Some(Self::window_proc),
    )?;

    // TODO: Check whether this can be done in a better way. Check out
    // SimpleClassicTheme.Taskbar project for potential implementation.
    unsafe { SetTimer(HWND(window as _), 1, 100, None) };

    Self::refresh_icons()?;

    Util::run_message_loop();

    Ok(())
  }

  extern "system" fn window_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
  ) -> LRESULT {
    match msg {
      WM_TIMER => {
        // Regain tray priority.
        let _ = unsafe {
          SetWindowPos(
            hwnd,
            HWND_TOPMOST,
            0,
            0,
            0,
            0,
            SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE,
          )
        };

        LRESULT(0)
      }
      WM_COPYDATA => Self::handle_copy_data(hwnd, msg, wparam, lparam),
      _ => {
        if Self::should_forward_message(msg) {
          Self::forward_message(hwnd, msg, wparam, lparam)
        } else {
          unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) }
        }
      }
    }
  }

  fn handle_copy_data(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
  ) -> LRESULT {
    // Extract `COPYDATASTRUCT` and return early if invalid.
    let Some(copy_data) =
      (unsafe { (lparam.0 as *const COPYDATASTRUCT).as_ref() })
    else {
      return LRESULT(0);
    };

    match copy_data.dwData {
      1 if !copy_data.lpData.is_null() => {
        let tray_message =
          unsafe { &*copy_data.lpData.cast::<ShellTrayMessage>() };

        let event_tx =
          TRAY_EVENT_TX.get().expect("Tray event sender not set.");

        if let Some(event) = tray_message.tray_event() {
          event_tx.send(event).expect("Failed to send tray event.");
        }

        Self::forward_message(hwnd, msg, wparam, lparam)
      }
      3 if !copy_data.lpData.is_null() => {
        let icon_identifier =
          unsafe { &*copy_data.lpData.cast::<WINNOTIFYICONIDENTIFIER>() };

        // TODO: Error handling.
        let cursor_pos = Util::cursor_position().unwrap();

        match icon_identifier.dwMessage {
          1 => LRESULT(Util::make_lparam(
            cursor_pos.0 as i16,
            cursor_pos.0 as i16,
          ) as _),
          2 => LRESULT(Util::make_lparam(
            cursor_pos.1 as i16 + 1,
            cursor_pos.1 as i16 + 1,
          ) as _),
          _ => LRESULT(0),
        }
      }
      _ => Self::forward_message(hwnd, msg, wparam, lparam),
    }
  }

  /// Refreshes the icons of the tray.
  ///
  /// Simulates the Windows taskbar being re-created. Some windows fail to
  /// re-add their icons, in which case it's an implementation error on
  /// their side. These windows that fail also do not re-add their icons
  /// to the Windows taskbar when `explorer.exe` is restarted ordinarily.
  fn refresh_icons() -> crate::Result<()> {
    tracing::info!(
      "Refreshing icons by sending `TaskbarCreated` message."
    );

    let msg = unsafe { RegisterWindowMessageW(w!("TaskbarCreated")) };

    if msg == 0 {
      return Err(windows::core::Error::from_win32().into());
    }

    unsafe { SendNotifyMessageW(HWND_BROADCAST, msg, None, None) }?;

    Ok(())
  }

  /// Whether a message should be forwarded to the real tray window.
  fn should_forward_message(msg: u32) -> bool {
    msg == WM_COPYDATA
      || msg == WM_ACTIVATEAPP
      || msg == WM_COMMAND
      || msg >= WM_USER
  }

  /// Forwards a message to the real tray window.
  fn forward_message(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
  ) -> LRESULT {
    tracing::info!(
      "Forwarding msg: {:#x} - {} to real tray window.",
      msg,
      msg
    );

    let Some(real_tray) = Util::tray_window(hwnd.0 as isize) else {
      tracing::warn!("No real tray found.");
      return unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) };
    };

    if msg > WM_USER {
      unsafe { PostMessageW(HWND(real_tray as _), msg, wparam, lparam) };
      unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) }
    } else {
      unsafe { SendMessageW(HWND(real_tray as _), msg, wparam, lparam) }
    }
  }
}
