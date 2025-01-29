use windows::Win32::UI::WindowsAndMessaging::{
  CreateWindowExW, RegisterClassW, CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT,
  WINDOW_EX_STYLE, WNDCLASSW, WNDPROC, WS_OVERLAPPEDWINDOW,
};
use windows_core::PCWSTR;

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

  pub fn to_wide(string: &str) -> Vec<u16> {
    string.encode_utf16().chain(std::iter::once(0)).collect()
  }
}
