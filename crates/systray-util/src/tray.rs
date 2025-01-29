use std::sync::OnceLock;

use tokio::sync::mpsc;
use windows::Win32::{
  Foundation::{HWND, LPARAM, LRESULT, WPARAM},
  System::DataExchange::COPYDATASTRUCT,
  UI::{
    Shell::{
      NOTIFYICONDATAW_0, NOTIFY_ICON_DATA_FLAGS,
      NOTIFY_ICON_INFOTIP_FLAGS, NOTIFY_ICON_STATE,
    },
    WindowsAndMessaging::{
      DefWindowProcW, SendMessageW, SetTimer, SetWindowPos, HICON,
      HWND_TOPMOST, SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOSIZE,
      WM_ACTIVATEAPP, WM_COMMAND, WM_COPYDATA, WM_TIMER, WM_USER,
    },
  },
};

use crate::Util;

#[repr(C)]
struct ShellTrayData {
  dw_magic: i32,
  dw_message: u32,
  nid: NotifyIconDataFixed,
}

#[repr(C)]
struct NotifyIconDataFixed {
  cb_size: u32,
  hwnd_raw: u32,
  uid: u32,
  flags: NOTIFY_ICON_DATA_FLAGS,
  callback_message: u32,
  hicon_raw: u32,
  sz_tip: [u16; 128],
  state: NOTIFY_ICON_STATE,
  state_mask: NOTIFY_ICON_STATE,
  sz_info: [u16; 256],
  union: NOTIFYICONDATAW_0,
  sz_info_title: [u16; 64],
  info_flags: NOTIFY_ICON_INFOTIP_FLAGS,
  guid: windows::core::GUID,
  balloon_icon: HICON,
}

enum PlatformEvent {
  // TODO: Add proper type.
  TrayUpdate(String),
}

/// Global instance of sender for platform events.
///
/// For use with window procedure.
static PLATFORM_EVENT_TX: OnceLock<mpsc::UnboundedSender<PlatformEvent>> =
  OnceLock::new();

pub fn run() -> crate::Result<()> {
  let (event_tx, event_rx) = mpsc::unbounded_channel();

  // Add the sender for platform events to global state.
  PLATFORM_EVENT_TX
    .set(event_tx)
    .expect("Platform event sender already set.");

  let window =
    Util::create_message_window("Shell_TrayWnd", Some(window_proc))?;

  // TODO: Check whether this can be done in a better way. Check out
  // SimpleClassicTheme.Taskbar project for potential implementation.
  unsafe { SetTimer(HWND(window as _), 1, 100, None) };

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
    WM_COPYDATA => {
      handle_copy_data(hwnd, wparam, lparam);
      forward_message(hwnd, wparam, lparam);
      LRESULT(0)
    }
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
    _ => {
      tracing::info!("msg: {:#x}", msg);
      if msg == WM_ACTIVATEAPP || msg == WM_COMMAND || msg >= WM_USER {
        forward_message(hwnd, wparam, lparam);
      }

      unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) }
    }
  }
}

fn handle_copy_data(
  hwnd: HWND,
  wparam: WPARAM,
  lparam: LPARAM,
) -> LRESULT {
  tracing::info!("Incoming `WM_COPYDATA` message.");

  // Extract COPYDATASTRUCT.
  let copy_data =
    match unsafe { (lparam.0 as *const COPYDATASTRUCT).as_ref() } {
      Some(data) => data,
      None => {
        tracing::warn!("Invalid COPYDATASTRUCT pointer.");
        forward_message(hwnd, wparam, lparam);
        return LRESULT(0);
      }
    };

  tracing::info!("COPYDATASTRUCT: {:?}", copy_data);

  // Process tray data if valid.
  if copy_data.dwData == 1 && !copy_data.lpData.is_null() {
    process_tray_data(hwnd, copy_data);
  }

  forward_message(hwnd, wparam, lparam);

  LRESULT(0)
}

fn process_tray_data(hwnd: HWND, copy_data: &COPYDATASTRUCT) {
  tracing::info!("Processing tray data.");

  // Get tray data
  let tray_data =
    unsafe { std::mem::transmute::<_, &ShellTrayData>(copy_data.lpData) };

  tracing::info!(
    "Icon data - hwnd: {:#x}, id: {}, flags: {:#x}",
    tray_data.nid.hwnd_raw,
    tray_data.nid.uid,
    tray_data.nid.flags.0
  );
}

/// Forwards a message to the real tray window.
fn forward_message(hwnd: HWND, wparam: WPARAM, lparam: LPARAM) {
  if let Some(real_tray) = Util::tray_window(hwnd.0 as isize) {
    tracing::debug!("Forwarding to real tray window: {:?}", real_tray);

    // TODO: Add error handling.
    let _ = unsafe {
      SendMessageW(HWND(real_tray as _), WM_COPYDATA, wparam, lparam)
    };
  } else {
    tracing::debug!("No real tray found.");
  }
}
