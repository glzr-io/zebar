use std::{mem, os::raw::c_void, sync::OnceLock, thread::JoinHandle};

use tokio::sync::mpsc;
use windows::{
  core::Error,
  Win32::{
    Foundation::{
      CloseHandle, HANDLE, HWND, INVALID_HANDLE_VALUE, LPARAM, LRESULT,
      WPARAM,
    },
    System::{
      DataExchange::COPYDATASTRUCT,
      Diagnostics::Debug::ReadProcessMemory,
      Memory::{
        CreateFileMappingW, MapViewOfFile, UnmapViewOfFile,
        VirtualAllocEx, VirtualFreeEx, FILE_MAP_ALL_ACCESS, MEM_COMMIT,
        MEM_RELEASE, PAGE_EXECUTE_READWRITE, PAGE_READWRITE,
      },
      Threading::{OpenProcess, PROCESS_ALL_ACCESS},
    },
    UI::{
      Shell::{
        NIF_GUID, NIF_ICON, NIF_MESSAGE, NIF_TIP, NIM_ADD, NIM_DELETE,
        NIM_MODIFY, NIM_SETVERSION, NOTIFYICONDATAW_0,
        NOTIFY_ICON_DATA_FLAGS, NOTIFY_ICON_INFOTIP_FLAGS,
        NOTIFY_ICON_MESSAGE, NOTIFY_ICON_STATE,
      },
      WindowsAndMessaging::{
        DefWindowProcW, GetWindowThreadProcessId, PostMessageW,
        RegisterWindowMessageW, SendMessageTimeoutW, SendMessageW,
        SendNotifyMessageW, SetTimer, SetWindowPos, HWND_BROADCAST,
        HWND_TOPMOST, SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOSIZE,
        WM_ACTIVATEAPP, WM_COMMAND, WM_COPYDATA, WM_TIMER, WM_USER,
      },
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
  icon_handle: u32,
  tooltip: [u16; 128],
  state: NOTIFY_ICON_STATE,
  state_mask: NOTIFY_ICON_STATE,
  size_info: [u16; 256],
  anonymous: NOTIFYICONDATAW_0,
  info_title: [u16; 64],
  info_flags: NOTIFY_ICON_INFOTIP_FLAGS,
  guid_item: windows_core::GUID,
  balloon_icon_handle: u32,
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
    let icon_handle = if self.icon_data.flags.0 & NIF_ICON.0 != 0 {
      Some(self.icon_data.icon_handle as isize)
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

    let callback_message = if self.icon_data.flags.0 & NIF_MESSAGE.0 != 0 {
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
      icon_handle,
      callback_message,
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
  pub icon_handle: Option<isize>,
  pub callback_message: Option<u32>,
  pub version: Option<u32>,
}

#[repr(C)]
#[derive(Debug)]
struct TbButton {
  bitmap: i32,
  command_id: i32,
  state: u8,
  style: u8,
  reserved: [u8; 6],
  data: usize,
  string_ptr: isize,
}

#[repr(C)]
#[derive(Debug)]
struct TrayItem {
  hwnd: isize,
  uid: u32,
  callback_message: u32,
  state: u32,
  version: u32,
  icon_handle: isize,
  icon_demote_timer_id: isize,
  user_pref: u32,
  last_sound_time: u32,
  exe_name: [u16; 260],
  icon_text: [u16; 260],
  num_seconds: u32,
  guid_item: windows_core::GUID,
}

const TB_BUTTONCOUNT: u32 = WM_USER + 24;
const TB_GETBUTTON: u32 = WM_USER + 23;

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

    let event_tx =
      TRAY_EVENT_TX.get().expect("Tray event sender not set.");

    if let Ok(icons) = Self::initial_tray_icons(window) {
      for icon in icons {
        if let Err(err) = event_tx.send(TrayEvent::IconAdd(icon)) {
          tracing::error!("Failed to send tray event: {:?}", err);
        }
      }
    } else {
      tracing::warn!(
        "Failed to retrieve initial tray icons. This is expected on W11."
      );
    }

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
          1 => LRESULT(Util::pack_i32(
            cursor_pos.0 as i16,
            cursor_pos.0 as i16,
          ) as _),
          2 => LRESULT(Util::pack_i32(
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

  pub fn initial_tray_icons(
    window_handle: isize,
  ) -> crate::Result<Vec<IconEventData>> {
    tracing::info!("Finding initial tray icons.");

    let tray = Util::find_tray_window(window_handle)
      .ok_or(crate::Error::TrayNotFound)?;

    let toolbars = [
      Util::find_tray_toolbar_window(tray),
      Util::find_overflow_toolbar_window(),
    ];

    // Get process handle.
    let mut process_id = 0u32;
    unsafe {
      GetWindowThreadProcessId(HWND(tray as _), Some(&mut process_id));
    }

    let process =
      unsafe { OpenProcess(PROCESS_ALL_ACCESS, false, process_id) }?;

    // Allocate memory in target process.
    let buffer = unsafe {
      VirtualAllocEx(
        process,
        None,
        mem::size_of::<TbButton>(),
        MEM_COMMIT,
        PAGE_READWRITE,
      )
    };

    if buffer.is_null() {
      return Err(crate::Error::Windows(Error::from_win32()));
    }

    let mut icons = Vec::new();

    for toolbar in toolbars.into_iter().flatten() {
      // Get number of tray icons.
      let count = unsafe {
        SendMessageW(HWND(toolbar as _), TB_BUTTONCOUNT, None, None)
      }
      .0 as usize;

      tracing::info!("Found {} buttons in toolbar.", count);

      // Read each button.
      for index in 0..count {
        // Get button info with timeout.
        unsafe {
          SendMessageW(
            HWND(toolbar as _),
            TB_GETBUTTON,
            WPARAM(index),
            LPARAM(buffer as isize),
          )
        };

        // Read shared memory containing the button data.
        let mut button: TbButton = unsafe { mem::zeroed() };
        unsafe {
          ReadProcessMemory(
            process,
            buffer,
            &mut button as *mut _ as _,
            mem::size_of::<TbButton>(),
            None,
          )
        }?;

        let mut tray_item: TrayItem = unsafe { mem::zeroed() };
        unsafe {
          ReadProcessMemory(
            process,
            button.data as _,
            &mut tray_item as *mut _ as _,
            mem::size_of::<TrayItem>(),
            None,
          )
        }?;

        tracing::info!("Found icon!!! {:?}", tray_item);

        let icon_data = IconEventData {
          uid: Some(tray_item.uid),
          window_handle: Some(tray_item.hwnd),
          guid: Some(uuid::Uuid::from_u128(tray_item.guid_item.to_u128())),
          tooltip: Some(
            String::from_utf16_lossy(&tray_item.icon_text)
              .trim_end_matches('\0')
              .to_string(),
          ),
          icon_handle: Some(tray_item.icon_handle),
          callback_message: Some(tray_item.callback_message),
          version: Some(tray_item.version),
        };

        icons.push(icon_data);
      }
    }

    // Cleanup.
    let _ = unsafe { VirtualFreeEx(process, buffer, 0, MEM_RELEASE) };
    let _ = unsafe { CloseHandle(process) };

    tracing::info!("Retrieved {} icons from system tray.", icons.len());

    Ok(icons)
  }

  fn get_button_icon_data(
    process: HANDLE,
    buffer: *mut c_void,
    toolbar: isize,
    index: usize,
  ) -> windows::core::Result<Option<IconEventData>> {
    // Send TB_GETBUTTON message
    unsafe {
      SendMessageW(
        HWND(toolbar as _),
        TB_GETBUTTON,
        WPARAM(index),
        LPARAM(buffer as isize),
      )
    };

    // Read button data
    let mut button: TbButton = unsafe { mem::zeroed() };
    let mut bytes_read = 0;
    let result = unsafe {
      ReadProcessMemory(
        process,
        buffer,
        &mut button as *mut _ as *mut c_void,
        mem::size_of::<TbButton>(),
        Some(&mut bytes_read),
      )
    }?;

    if bytes_read == 0 || button.data == 0 {
      return Ok(None);
    }

    // Read tray item data
    let mut tray_item: TrayItem = unsafe { mem::zeroed() };
    let mut bytes_read = 0;
    let result = unsafe {
      ReadProcessMemory(
        process,
        button.data as *mut c_void,
        &mut tray_item as *mut _ as *mut c_void,
        mem::size_of::<TrayItem>(),
        Some(&mut bytes_read),
      )
    }?;

    if bytes_read == 0 {
      return Ok(None);
    }

    // Convert to IconEventData
    Ok(Some(IconEventData {
      uid: Some(tray_item.uid),
      window_handle: Some(tray_item.hwnd),
      guid: Some(uuid::Uuid::from_u128(tray_item.guid_item.to_u128())),
      tooltip: Some(
        String::from_utf16_lossy(&tray_item.icon_text)
          .trim_end_matches('\0')
          .to_string(),
      ),
      icon_handle: Some(tray_item.icon_handle),
      callback_message: Some(tray_item.callback_message),
      version: Some(tray_item.version),
    }))
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

    let Some(real_tray) = Util::find_tray_window(hwnd.0 as isize) else {
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
