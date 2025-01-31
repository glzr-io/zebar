use std::{sync::OnceLock, thread::JoinHandle};

use image::RgbaImage;
use tokio::sync::mpsc;
use windows::Win32::{
  Foundation::{HWND, LPARAM, LRESULT, WPARAM},
  System::DataExchange::COPYDATASTRUCT,
  UI::{
    Shell::{
      NIM_ADD, NIM_DELETE, NIM_MODIFY, NIM_SETVERSION, NOTIFYICONDATAW_0,
      NOTIFY_ICON_DATA_FLAGS, NOTIFY_ICON_INFOTIP_FLAGS,
      NOTIFY_ICON_MESSAGE, NOTIFY_ICON_STATE,
    },
    WindowsAndMessaging::{
      DefWindowProcW, PostMessageW, RegisterWindowMessageW, SendMessageW,
      SendNotifyMessageW, SetTimer, SetWindowPos, HICON, HWND_BROADCAST,
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
  icon_data: NOTIFYICONDATAW,
  version: u32,
}

/// Contains the data for a system tray icon.
///
/// When `Shell_NotifyIcon` sends its message to `Shell_Traywnd`, it
/// actually uses a 32-bit handle for the hwnd. This makes it slightly
/// different than the `windows` crate's `NOTIFYICONDATAW` type.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NOTIFYICONDATAW {
  pub cbSize: u32,
  pub hWnd: u32,
  pub uID: u32,
  pub uFlags: NOTIFY_ICON_DATA_FLAGS,
  pub uCallbackMessage: u32,
  pub hIcon: u32,
  pub szTip: [u16; 128],
  pub dwState: NOTIFY_ICON_STATE,
  pub dwStateMask: NOTIFY_ICON_STATE,
  pub szInfo: [u16; 256],
  pub Anonymous: NOTIFYICONDATAW_0,
  pub szInfoTitle: [u16; 64],
  pub dwInfoFlags: NOTIFY_ICON_INFOTIP_FLAGS,
  pub guidItem: windows_core::GUID,
  pub hBalloonIcon: HICON,
}

impl ShellTrayMessage {
  fn tray_event(&self) -> crate::Result<Option<TrayEvent>> {
    let event = match NOTIFY_ICON_MESSAGE(self.message_type) {
      NIM_ADD => Some(TrayEvent::IconAdd(self.icon_data()?)),
      NIM_MODIFY | NIM_SETVERSION => {
        Some(TrayEvent::IconUpdate(self.icon_data()?))
      }
      NIM_DELETE => Some(TrayEvent::IconRemove(self.icon_data.uID)),
      _ => None,
    };

    Ok(event)
  }

  fn icon_data(&self) -> crate::Result<IconData> {
    let icon_data = IconData {
      uid: self.icon_data.uID,
      window_handle: self.icon_data.hWnd as isize,
      tooltip: String::from_utf16_lossy(&self.icon_data.szTip),
      icon: Util::icon_to_image(self.icon_data.hIcon)?,
      callback: self.icon_data.uCallbackMessage,
    };

    Ok(icon_data)
  }
}

/// Events emitted by the spy window.
#[derive(Debug)]
pub(crate) enum TrayEvent {
  IconAdd(IconData),
  IconUpdate(IconData),
  IconRemove(u32),
}

#[derive(Debug)]
pub(crate) struct IconData {
  pub uid: u32,
  pub window_handle: isize,
  pub tooltip: String,
  pub icon: RgbaImage,
  pub callback: u32,
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
    // Regain tray priority.
    if msg == WM_TIMER {
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

      return LRESULT(0);
    }

    tracing::info!("msg: {:#x} {}", msg, msg);

    if msg == WM_COPYDATA {
      if let Err(err) = Self::handle_copy_data(hwnd, wparam, lparam) {
        tracing::warn!(
          "Failed to handle `WM_COPYDATA` message: {:?}",
          err
        );
      }
    }

    if Self::should_forward_message(msg) {
      Self::forward_message(hwnd, msg, wparam, lparam)
      // if let Err(err) = forward_message(hwnd, msg, wparam, lparam) {
      //   tracing::warn!(
      //     "Failed to forward message to tray window: {:?}",
      //     err
      //   );
      // }
    } else {
      unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) }
    }
  }

  fn handle_copy_data(
    hwnd: HWND,
    wparam: WPARAM,
    lparam: LPARAM,
  ) -> crate::Result<()> {
    tracing::info!("Incoming `WM_COPYDATA` message.");

    // Extract `COPYDATASTRUCT` and return early if invalid.
    let copy_data =
      (unsafe { (lparam.0 as *const COPYDATASTRUCT).as_ref() })
        .ok_or(crate::Error::CopyDataInvalid)?;

    tracing::info!("COPYDATASTRUCT: {:?}", copy_data);

    // Process tray data if valid.
    if copy_data.dwData == 1 && !copy_data.lpData.is_null() {
      tracing::info!("Processing tray data.");

      let tray_message =
        unsafe { &*copy_data.lpData.cast::<ShellTrayMessage>() };

      let event_tx =
        TRAY_EVENT_TX.get().expect("Tray event sender not set.");

      if let Some(event) = tray_message.tray_event()? {
        event_tx.send(event).expect("Failed to send tray event.");
      }
    }

    Ok(())
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

    let Some(real_tray) = Util::tray_window_2(hwnd.0 as isize) else {
      tracing::warn!("No real tray found.");
      return LRESULT(0);
    };

    if msg > WM_USER || msg == WM_COPYDATA {
      unsafe { PostMessageW(HWND(real_tray as _), msg, wparam, lparam) };
      unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) }
    } else {
      unsafe { SendMessageW(HWND(real_tray as _), msg, wparam, lparam) }
    }
  }
}
