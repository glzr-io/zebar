use std::sync::OnceLock;

use tokio::sync::mpsc;
use windows::Win32::{
  Foundation::{HWND, LPARAM, LRESULT, WPARAM},
  System::DataExchange::COPYDATASTRUCT,
  UI::{
    Shell::{
      NIM_ADD, NIM_DELETE, NIM_MODIFY, NIM_SETVERSION, NOTIFYICONDATAW,
      NOTIFY_ICON_MESSAGE,
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

use crate::{NativeWindow, Util};

/// Tray message sent to `Shell_TrayWnd` and intercepted by our spy window.
#[repr(C)]
struct ShellTrayMessage {
  magic_number: i32,
  message_type: u32,
  icon_data: NOTIFYICONDATAW,
  version: u32,
}

/// Events emitted by the spy window.
#[derive(Debug)]
enum TrayEvent {
  IconAdd(String),
  IconUpdate(String),
  IconRemove(String),
}

/// Global instance of sender for tray events.
///
/// For use with window procedure.
static TRAY_EVENT_TX: OnceLock<mpsc::UnboundedSender<TrayEvent>> =
  OnceLock::new();

pub fn run() -> crate::Result<()> {
  let (event_tx, event_rx) = mpsc::unbounded_channel();

  // Add the sender for tray events to global state.
  TRAY_EVENT_TX
    .set(event_tx)
    .expect("Tray event sender already set.");

  let window =
    Util::create_message_window("Shell_TrayWnd", Some(window_proc))?;

  // TODO: Check whether this can be done in a better way. Check out
  // SimpleClassicTheme.Taskbar project for potential implementation.
  unsafe { SetTimer(HWND(window as _), 1, 100, None) };

  refresh_icons()?;

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
    if let Err(err) = handle_copy_data(hwnd, wparam, lparam) {
      tracing::warn!("Failed to handle `WM_COPYDATA` message: {:?}", err);
    }
  }

  if should_forward_message(msg) {
    forward_message(hwnd, msg, wparam, lparam)
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

    match NOTIFY_ICON_MESSAGE(tray_message.message_type) {
      NIM_ADD | NIM_MODIFY => {
        let window =
          NativeWindow::new(tray_message.icon_data.hWnd.0 as isize);
        let process_name = window.process_name().unwrap_or_default();

        tracing::info!(
          "Add/Modify data - hwnd: {:#x}, process: {}, tooltip: {}, message: {}",
          tray_message.icon_data.hWnd.0 as isize,
          process_name,
          String::from_utf16_lossy(&tray_message.icon_data.szTip),
          tray_message.message_type
        );
      }
      NIM_DELETE => {
        tracing::info!(
          "Delete data - hwnd: {:#x}",
          tray_message.icon_data.hWnd.0 as isize
        );
      }
      NIM_SETVERSION => {
        tracing::info!(
          "Set version - hwnd: {:#x}, version: {}",
          tray_message.icon_data.hWnd.0 as isize,
          tray_message.version
        );
      }
      _ => {
        tracing::info!("Other message: {:#x}", tray_message.message_type);
      }
    }
  }

  Ok(())
}

/// Refreshes the icons of the tray.
///
/// Simulates the Windows taskbar being re-created. Some windows fail to
/// re-add their icons, in which case it's an implementation error on their
/// side. These windows that fail also do not re-add their icons to the
/// Windows taskbar when `explorer.exe` is restarted ordinarily.
fn refresh_icons() -> crate::Result<()> {
  tracing::info!("Refreshing icons by sending `TaskbarCreated` message.");

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
