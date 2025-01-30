use std::{
  collections::{HashMap, VecDeque},
  sync::{LazyLock, Mutex, OnceLock},
};

use tokio::sync::mpsc;
use windows::Win32::{
  Foundation::{HWND, LPARAM, LRESULT, WPARAM},
  System::DataExchange::COPYDATASTRUCT,
  UI::{
    Shell::{
      NIM_ADD, NOTIFYICONDATAW_0, NOTIFY_ICON_DATA_FLAGS,
      NOTIFY_ICON_INFOTIP_FLAGS, NOTIFY_ICON_STATE,
    },
    WindowsAndMessaging::{
      DefWindowProcW, PeekMessageW, PostMessageW, SendMessageW, SetTimer,
      SetWindowPos, HICON, HWND_TOPMOST, MSG, PM_NOREMOVE, SWP_NOACTIVATE,
      SWP_NOMOVE, SWP_NOSIZE, WM_ACTIVATEAPP, WM_COMMAND, WM_COPYDATA,
      WM_TIMER, WM_USER,
    },
  },
};

use crate::Util;

#[repr(C)]
struct ShellTrayData {
  dw_magic: i32,
  dw_message: u32,
  nid: NotifyIconDataFixed,
  u_version: u32,
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

// Track messages that are currently being processed to detect recursion
static PROCESSING_MESSAGES: LazyLock<
  Mutex<HashMap<MessageKey, Option<LRESULT>>>,
> = LazyLock::new(|| Mutex::new(HashMap::new()));

#[derive(Hash, Eq, PartialEq, Clone)]
struct MessageKey {
  hwnd: isize,
  msg: u32,
  wparam: usize,
  lparam: isize,
}

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
    process_tray_data(hwnd, copy_data);
  }

  Ok(())
}

fn process_tray_data(hwnd: HWND, copy_data: &COPYDATASTRUCT) {
  tracing::info!("Processing tray data.");

  // NIM_ADD
  // Get tray data
  let tray_data =
    unsafe { std::mem::transmute::<_, &ShellTrayData>(copy_data.lpData) };

  tracing::info!(
    "Icon data - hwnd: {:#x}, id: {}, flags: {:#x}, message: {}, version: {}",
    tray_data.nid.hwnd_raw,
    tray_data.nid.uid,
    tray_data.nid.flags.0,
    tray_data.dw_message,
    tray_data.u_version
  );
}

/// Determines whether a message should be forwarded to the real tray
/// window.
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
  let message_key = MessageKey {
    hwnd: hwnd.0 as isize,
    msg,
    wparam: wparam.0,
    lparam: lparam.0,
  };

  start_send_message(message_key.clone());

  loop {
    match message_response(&message_key) {
      Some(res) => return res,
      None => {
        let mut msg = MSG::default();

        // Send off other messages while waiting for response.
        if unsafe { PeekMessageW(&mut msg, None, 0, 0, PM_NOREMOVE) }
          .as_bool()
        {
          if should_forward_message(msg.message) {
            start_send_message(MessageKey {
              hwnd: msg.hwnd.0 as isize,
              msg: msg.message,
              wparam: msg.wParam.0,
              lparam: msg.lParam.0,
            });
          }
        }

        std::thread::sleep(std::time::Duration::from_millis(16));
      }
    }
  }
}

fn message_response(message_key: &MessageKey) -> Option<LRESULT> {
  let mut messages = PROCESSING_MESSAGES.lock().unwrap();
  messages.remove(message_key).flatten()
}

fn start_send_message(message_key: MessageKey) {
  let mut messages = PROCESSING_MESSAGES.lock().unwrap();

  if messages.contains_key(&message_key) {
    return;
  }

  messages.insert(message_key.clone(), None);

  let real_tray = Util::tray_window_2(message_key.hwnd).unwrap();

  tracing::info!(
    "Forwarding msg: {:#x} - {} to real tray window.",
    message_key.msg,
    message_key.msg
  );

  std::thread::spawn(move || {
    let res = unsafe {
      SendMessageW(
        HWND(real_tray as _),
        message_key.msg,
        WPARAM(message_key.wparam),
        LPARAM(message_key.lparam),
      )
    };

    tracing::info!(
      "Received response for msg: {:#x} - {} from real tray window.",
      message_key.msg,
      message_key.msg
    );

    // Modify the message to include the result.
    let mut messages = PROCESSING_MESSAGES.lock().unwrap();
    messages.insert(message_key, Some(res));
  });
}
