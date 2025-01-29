use std::{os::windows::io::AsRawHandle, thread::JoinHandle};

use windows::Win32::{
  Foundation::{BOOL, HANDLE, HWND, LPARAM, WPARAM},
  System::Threading::GetThreadId,
  UI::WindowsAndMessaging::{
    CreateWindowExW, DispatchMessageW, EnumWindows, FindWindowExW,
    FindWindowW, GetClassNameW, GetMessageW, PostThreadMessageW,
    RegisterClassW, TranslateMessage, CS_HREDRAW, CS_VREDRAW,
    CW_USEDEFAULT, MSG, WINDOW_EX_STYLE, WM_QUIT, WNDCLASSW, WNDPROC,
    WS_OVERLAPPEDWINDOW,
  },
};
use windows_core::{w, PCWSTR, PWSTR};

pub type WindowProcedure = WNDPROC;

pub struct Util;

impl Util {
  /// Creates a hidden message window.
  ///
  /// Returns a handle to the created window.
  pub fn create_message_window(
    class_name: &str,
    window_procedure: WindowProcedure,
  ) -> crate::Result<isize> {
    let class_name = Self::to_wide(class_name);

    let class = WNDCLASSW {
      lpszClassName: PCWSTR::from_raw(class_name.as_ptr()),
      style: CS_HREDRAW | CS_VREDRAW,
      lpfnWndProc: window_procedure,
      ..Default::default()
    };

    let class_atom = unsafe { RegisterClassW(&class) };

    if class_atom == 0 {
      return Err(crate::Error::MessageWindowCreationFailed);
    }

    let handle = unsafe {
      CreateWindowExW(
        WINDOW_EX_STYLE::default(),
        PCWSTR::from_raw(class_name.as_ptr()),
        PCWSTR::from_raw(class_name.as_ptr()),
        WS_OVERLAPPEDWINDOW,
        CW_USEDEFAULT,
        CW_USEDEFAULT,
        CW_USEDEFAULT,
        CW_USEDEFAULT,
        None,
        None,
        class.hInstance,
        None,
      )
    }?;

    Ok(handle.0 as isize)
  }

  /// Starts a message loop on the current thread.
  ///
  /// This function will block until the message loop is killed. Use
  /// `Util::kill_message_loop` to terminate the message loop.
  pub fn run_message_loop() {
    let mut msg = MSG::default();

    loop {
      if unsafe { GetMessageW(&mut msg, None, 0, 0) }.as_bool() {
        unsafe {
          TranslateMessage(&msg);
          DispatchMessageW(&msg);
        }
      } else {
        break;
      }
    }
  }

  /// Gracefully terminates the message loop on the given thread.
  pub fn kill_message_loop<T>(
    thread: &JoinHandle<T>,
  ) -> crate::Result<()> {
    let handle = thread.as_raw_handle();
    let handle = HANDLE(handle);
    let thread_id = unsafe { GetThreadId(handle) };

    unsafe {
      PostThreadMessageW(
        thread_id,
        WM_QUIT,
        WPARAM::default(),
        LPARAM::default(),
      )
    }?;

    Ok(())
  }

  /// Converts a string to a wide string.
  pub fn to_wide(string: &str) -> Vec<u16> {
    string.encode_utf16().chain(std::iter::once(0)).collect()
  }

  /// TODO: Could be significantly simplified.
  pub fn tray_window(spy_window: isize) -> Option<isize> {
    let real_tray =
      unsafe { FindWindowW(w!("Shell_TrayWnd"), PWSTR::null()) }.ok()?;

    if real_tray != HWND(spy_window as _) {
      if let Ok(_) = unsafe {
        FindWindowExW(
          real_tray,
          HWND(std::ptr::null_mut()),
          w!("TrayNotifyWnd"),
          PWSTR::null(),
        )
      } {
        return Some(real_tray.0 as isize);
      }
    }

    let mut info = FindRealTrayInfo {
      spy_window: HWND(spy_window as _),
      result_window: HWND(std::ptr::null_mut()),
    };

    unsafe extern "system" fn enum_proc(
      hwnd: HWND,
      lparam: LPARAM,
    ) -> BOOL {
      let info = lparam.0 as *mut FindRealTrayInfo;
      let mut class_name = [0u16; 16];

      if GetClassNameW(hwnd, &mut class_name) != 0 {
        let class_str = String::from_utf16_lossy(
          &class_name
            [..class_name.iter().position(|&x| x == 0).unwrap_or(0)],
        );
        if class_str == "Shell_TrayWnd" && hwnd != (*info).spy_window {
          if let Ok(_) = FindWindowExW(
            hwnd,
            HWND(std::ptr::null_mut()),
            w!("TrayNotifyWnd"),
            PWSTR::null(),
          ) {
            (*info).result_window = hwnd;
            return false.into();
          }
        }
      }
      true.into()
    }

    let _ = unsafe {
      EnumWindows(Some(enum_proc), LPARAM(&mut info as *mut _ as isize))
    };

    if !info.result_window.0.is_null() {
      Some(info.result_window.0 as isize)
    } else {
      let progman =
        unsafe { FindWindowW(w!("Progman"), PWSTR::null()) }.ok()?;
      let tray = unsafe {
        FindWindowExW(
          progman,
          HWND(std::ptr::null_mut()),
          w!("Shell_TrayWnd"),
          PWSTR::null(),
        )
      }
      .ok()?;

      if tray != HWND(spy_window as _) {
        if let Ok(_) = unsafe {
          FindWindowExW(
            tray,
            HWND(std::ptr::null_mut()),
            w!("TrayNotifyWnd"),
            PWSTR::null(),
          )
        } {
          Some(tray.0 as isize)
        } else {
          None
        }
      } else {
        None
      }
    }
  }
}

struct FindRealTrayInfo {
  spy_window: HWND,
  result_window: HWND,
}
