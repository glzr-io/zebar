use std::{
  mem::MaybeUninit, os::windows::io::AsRawHandle, ptr::addr_of_mut,
  thread::JoinHandle,
};

use image::RgbaImage;
use windows::Win32::{
  Foundation::{BOOL, HANDLE, HWND, LPARAM, WPARAM},
  Graphics::Gdi::{
    DeleteObject, GetBitmapBits, GetDC, GetDIBits, GetObjectW, ReleaseDC,
    BITMAP, BITMAPINFO, BITMAPINFOHEADER, BI_RGB, DIB_RGB_COLORS, HBITMAP,
    HGDIOBJ,
  },
  System::Threading::GetThreadId,
  UI::WindowsAndMessaging::{
    CreateWindowExW, DispatchMessageW, EnumWindows, FindWindowExW,
    FindWindowW, GetClassNameW, GetIconInfo, GetMessageW,
    PostThreadMessageW, RegisterClassW, TranslateMessage, CS_HREDRAW,
    CS_VREDRAW, CW_USEDEFAULT, HICON, ICONINFO, MSG, WINDOW_EX_STYLE,
    WM_QUIT, WNDCLASSW, WNDPROC, WS_EX_APPWINDOW, WS_EX_TOOLWINDOW,
    WS_EX_TOPMOST, WS_OVERLAPPEDWINDOW,
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
        WS_EX_TOOLWINDOW | WS_EX_APPWINDOW | WS_EX_TOPMOST,
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

  /// Creates an `LPARAM` from low-order and high-order words. This is
  /// equivalent to the Win32 `MAKELPARAM` macro.
  pub fn make_lparam(low: u16, high: u16) -> u32 {
    low as u32 | ((high as u32) << 16)
  }

  /// Converts a Windows icon to a sendable image.
  pub fn icon_to_image(icon: u32) -> crate::Result<RgbaImage> {
    let mut icon_info = ICONINFO::default();
    unsafe { GetIconInfo(HICON(icon as _), &mut icon_info) }?;
    unsafe { DeleteObject(icon_info.hbmMask) }.ok()?;

    let mut bitmap = BITMAP::default();
    let bitmap_res = unsafe {
      GetObjectW(
        icon_info.hbmColor,
        std::mem::size_of::<BITMAP>() as i32,
        Some(&mut bitmap as *mut _ as _),
      )
    };

    if bitmap_res == 0 {
      unsafe { DeleteObject(icon_info.hbmColor) }.ok()?;
      return Err(windows::core::Error::from_win32().into());
    }

    let width_u32 = u32::try_from(bitmap.bmWidth)?;
    let height_u32 = u32::try_from(bitmap.bmHeight)?;
    let width_usize = usize::try_from(bitmap.bmWidth)?;
    let height_usize = usize::try_from(bitmap.bmHeight)?;

    let mut buffer: Vec<u8> = Vec::with_capacity(
      width_usize
        .checked_mul(height_usize)
        .and_then(|size| size.checked_mul(4))
        .ok_or(crate::Error::IconConversionFailed)?,
    );

    let dc = unsafe { GetDC(None) };
    if dc.is_invalid() {
      unsafe { DeleteObject(icon_info.hbmColor) }.ok()?;
      return Err(windows::core::Error::from_win32().into());
    }

    let mut bi = BITMAPINFO {
      bmiHeader: BITMAPINFOHEADER {
        biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
        biWidth: bitmap.bmWidth,
        biHeight: -bitmap.bmHeight.abs(),
        biPlanes: 1,
        biBitCount: 32,
        ..Default::default()
      },
      ..Default::default()
    };

    let result = unsafe {
      GetDIBits(
        dc,
        icon_info.hbmColor,
        0,
        height_u32,
        Some(buffer.as_mut_ptr().cast()),
        addr_of_mut!(bi).cast(),
        DIB_RGB_COLORS,
      )
    };

    if result == 0 {
      unsafe { DeleteObject(icon_info.hbmColor) }.ok()?;
      unsafe { ReleaseDC(None, dc) };
      return Err(windows::core::Error::from_win32().into());
    }

    unsafe { buffer.set_len(buffer.capacity()) };
    unsafe { ReleaseDC(None, dc) };
    unsafe { DeleteObject(icon_info.hbmColor) }.ok()?;

    // Convert BGR to RGB. The red and blue channels get swapped.
    for chunk in buffer.chunks_exact_mut(4) {
      let [blue, _, red, _] = chunk else {
        unreachable!()
      };

      std::mem::swap(blue, red);
    }

    RgbaImage::from_vec(width_u32, height_u32, buffer)
      .ok_or(crate::Error::IconConversionFailed)
  }

  /// Finds the Windows tray window, optionally ignoring a specific window
  /// handle.
  pub fn tray_window_2(hwnd_ignore: isize) -> Option<isize> {
    let mut taskbar_hwnd = unsafe {
      FindWindowW(
        windows::core::PCWSTR::from_raw(
          "Shell_TrayWnd\0"
            .encode_utf16()
            .collect::<Vec<_>>()
            .as_ptr(),
        ),
        windows::core::PCWSTR::null(),
      )
    }
    .ok()?;

    if hwnd_ignore != 0 {
      while taskbar_hwnd == HWND(hwnd_ignore as _) {
        taskbar_hwnd = unsafe {
          FindWindowExW(
            HWND::default(),
            taskbar_hwnd,
            windows::core::PCWSTR::from_raw(
              "Shell_TrayWnd\0"
                .encode_utf16()
                .collect::<Vec<_>>()
                .as_ptr(),
            ),
            windows::core::PCWSTR::null(),
          )
        }
        .ok()?;
      }
    }

    Some(taskbar_hwnd.0 as isize)
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
