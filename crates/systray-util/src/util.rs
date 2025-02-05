use std::{
  os::windows::io::AsRawHandle, ptr::addr_of_mut, thread::JoinHandle,
};

use image::RgbaImage;
use windows::Win32::{
  Foundation::{HANDLE, HWND, LPARAM, POINT, WPARAM},
  Graphics::Gdi::{
    DeleteObject, GetDC, GetDIBits, GetObjectW, ReleaseDC, BITMAP,
    BITMAPINFO, BITMAPINFOHEADER, DIB_RGB_COLORS,
  },
  System::Threading::GetThreadId,
  UI::WindowsAndMessaging::{
    CreateWindowExW, DispatchMessageW, FindWindowExW, FindWindowW,
    GetCursorPos, GetIconInfo, GetMessageW, PostThreadMessageW,
    RegisterClassW, TranslateMessage, CS_HREDRAW, CS_VREDRAW,
    CW_USEDEFAULT, HICON, ICONINFO, MSG, WM_QUIT, WNDCLASSW, WNDPROC,
    WS_EX_APPWINDOW, WS_EX_TOOLWINDOW, WS_EX_TOPMOST, WS_OVERLAPPEDWINDOW,
  },
};
use windows_core::{w, PCWSTR};

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

  /// Packs two 16-bit values into a 32-bit value. This is commonly used
  /// for `WPARAM` and `LPARAM` values.
  ///
  /// Equivalent to the Win32 `MAKELPARAM` and `MAKEWPARAM` macros.
  pub fn pack_i32(low: i16, high: i16) -> i32 {
    low as i32 | ((high as i32) << 16)
  }

  /// Gets the mouse position in screen coordinates.
  pub fn cursor_position() -> crate::Result<(i32, i32)> {
    let mut point = POINT { x: 0, y: 0 };
    unsafe { GetCursorPos(&mut point) }?;
    Ok((point.x, point.y))
  }

  /// Converts a Windows icon to a sendable image.
  pub fn icon_to_image(icon: isize) -> crate::Result<RgbaImage> {
    let mut icon_info = ICONINFO::default();
    unsafe { GetIconInfo(HICON(icon as _), &mut icon_info) }?;

    let mut bitmap = BITMAP::default();
    let bitmap_res = unsafe {
      GetObjectW(
        icon_info.hbmColor,
        std::mem::size_of::<BITMAP>() as i32,
        Some(&mut bitmap as *mut _ as _),
      )
    };

    if bitmap_res == 0 {
      let error = windows::core::Error::from_win32();
      unsafe { DeleteObject(icon_info.hbmMask) }.ok()?;
      unsafe { DeleteObject(icon_info.hbmColor) }.ok()?;
      return Err(error.into());
    }

    let width_u32 = u32::try_from(bitmap.bmWidth)?;
    let height_u32 = u32::try_from(bitmap.bmHeight)?;
    let width_usize = usize::try_from(bitmap.bmWidth)?;
    let height_usize = usize::try_from(bitmap.bmHeight)?;

    let buffer_size = width_usize
      .checked_mul(height_usize)
      .and_then(|size| size.checked_mul(4))
      .ok_or(crate::Error::IconConversionFailed)?;

    // For most icons, we only need the color bitmap. However, certain
    // icons (e.g. the NVIDIA settings app) require both mask and color
    // bitmaps.
    let mut color_buffer = vec![0u8; buffer_size];
    let mut mask_buffer = vec![0u8; buffer_size];

    let dc = unsafe { GetDC(None) };
    if dc.is_invalid() {
      let error = windows::core::Error::from_win32();
      unsafe { DeleteObject(icon_info.hbmMask) }.ok()?;
      unsafe { DeleteObject(icon_info.hbmColor) }.ok()?;
      return Err(error.into());
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

    // Get color bitmap data.
    let color_result = unsafe {
      GetDIBits(
        dc,
        icon_info.hbmColor,
        0,
        height_u32,
        Some(color_buffer.as_mut_ptr().cast()),
        addr_of_mut!(bi).cast(),
        DIB_RGB_COLORS,
      )
    };

    // Get mask bitmap data.
    let mask_result = unsafe {
      GetDIBits(
        dc,
        icon_info.hbmMask,
        0,
        height_u32,
        Some(mask_buffer.as_mut_ptr().cast()),
        addr_of_mut!(bi).cast(),
        DIB_RGB_COLORS,
      )
    };

    unsafe { ReleaseDC(None, dc) };
    unsafe { DeleteObject(icon_info.hbmMask) }.ok()?;
    unsafe { DeleteObject(icon_info.hbmColor) }.ok()?;

    if color_result == 0 || mask_result == 0 {
      return Err(windows::core::Error::from_win32().into());
    }

    // Masks are a legacy way of handling transparency before alpha
    // channels became standard.  Either white/black is used to
    // represent transparency. Modern icons will have partial transparency
    // throughout the icon from anti-aliasing
    let is_mask_based =
      color_buffer.chunks_exact(4).all(|chunk| chunk[3] == 0);

    // Combine color and mask data. We also need to convert BGR to RGB,
    // meaning that the red and blue channels get swapped.
    for (index, chunk) in color_buffer.chunks_exact_mut(4).enumerate() {
      // Get mask bit (every 4th byte since we're reading as 32-bit).
      let mask_alpha = mask_buffer[index * 4];

      // Swap BGR to RGB.
      chunk.swap(0, 2);
      if is_mask_based {
        if mask_alpha == 255 {
          // Make pixel transparent.
          chunk[3] = 0;
        } else if chunk[0] != 0 || chunk[1] != 0 || chunk[2] != 0 {
          // Make pixel solid.
          chunk[3] = 255;
        }
      }
    }

    RgbaImage::from_vec(width_u32, height_u32, color_buffer)
      .ok_or(crate::Error::IconConversionFailed)
  }

  /// Finds the Windows tray window, ignoring a specific window handle.
  pub fn find_tray_window(hwnd_ignore: isize) -> Option<isize> {
    let mut taskbar_hwnd =
      unsafe { FindWindowW(w!("Shell_TrayWnd"), None) }.ok()?;

    if hwnd_ignore != 0 {
      while taskbar_hwnd == HWND(hwnd_ignore as _) {
        taskbar_hwnd = unsafe {
          FindWindowExW(
            HWND::default(),
            taskbar_hwnd,
            w!("Shell_TrayWnd"),
            None,
          )
        }
        .ok()?;
      }
    }

    Some(taskbar_hwnd.0 as isize)
  }

  /// Finds the toolbar window (contains tray icons) within the given tray
  /// window.
  pub fn find_tray_toolbar_window(tray_handle: isize) -> Option<isize> {
    let notify = unsafe {
      FindWindowExW(
        HWND(tray_handle as _),
        None,
        w!("TrayNotifyWnd"),
        None,
      )
    }
    .ok()?;
    tracing::info!("Found TrayNotifyWnd: {:?}", notify);

    let pager =
      unsafe { FindWindowExW(notify, None, w!("SysPager"), None) }.ok()?;
    tracing::info!("Found SysPager: {:?}", pager);

    let toolbar =
      unsafe { FindWindowExW(pager, None, w!("ToolbarWindow32"), None) }
        .ok()?;
    tracing::info!("Found ToolbarWindow32: {:?}", toolbar);

    Some(toolbar.0 as isize)
  }

  /// Finds the toolbar window (contains tray icons) for overflowed icons.
  /// This is the window accessed via the chevron button in the Windows
  /// taskbar.
  pub fn find_overflow_toolbar_window() -> Option<isize> {
    let notify =
      unsafe { FindWindowW(w!("NotifyIconOverflowWindow"), None) }.ok()?;

    let toolbar =
      unsafe { FindWindowExW(notify, None, w!("ToolbarWindow32"), None) }
        .ok()?;

    Some(toolbar.0 as isize)
  }
}
