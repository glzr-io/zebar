use windows::Win32::{
  Globalization::{LCIDToLocaleName, LOCALE_ALLOW_NEUTRAL_NAMES},
  System::SystemServices::LOCALE_NAME_MAX_LENGTH,
  UI::{
    Input::KeyboardAndMouse::*, TextServices::HKL, WindowsAndMessaging::*,
  },
};

pub struct Language {}

impl Language {
  pub fn new() -> Self {
    Language {}
  }

  pub fn get_current_language(&self) -> String {
    let keyboard_layout = unsafe {
      GetKeyboardLayout(GetWindowThreadProcessId(
        GetForegroundWindow(),
        None,
      ))
    };
    let lang_id = Self::loword(keyboard_layout);

    let mut locale_name = [0; LOCALE_NAME_MAX_LENGTH as usize];

    let result = unsafe {
      LCIDToLocaleName(
        lang_id,
        Some(&mut locale_name),
        LOCALE_ALLOW_NEUTRAL_NAMES,
      )
    };

    let mut actual_name = String::from("unknown");
    if result > 0 {
      actual_name =
        String::from_utf16_lossy(&locale_name[..result as usize]);
    }

    actual_name
  }

  fn loword(l: HKL) -> u32 {
    (l.0 as u32) & 0xffff
  }
}
