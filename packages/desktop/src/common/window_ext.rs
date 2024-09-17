use anyhow::Context;
#[cfg(target_os = "macos")]
use cocoa::{
  appkit::{NSMainMenuWindowLevel, NSWindow},
  base::id,
};
use tauri::{Runtime, Window};
#[cfg(target_os = "windows")]
use windows::Win32::UI::WindowsAndMessaging::{
  SetWindowLongPtrW, GWL_EXSTYLE, WS_EX_APPWINDOW, WS_EX_TOOLWINDOW,
};

pub trait WindowExt {
  #[cfg(target_os = "macos")]
  fn set_above_menu_bar(&self) -> anyhow::Result<()>;

  #[cfg(target_os = "windows")]
  fn set_tool_window(&self, enable: bool) -> anyhow::Result<()>;
}

impl<R: Runtime> WindowExt for Window<R> {
  #[cfg(target_os = "macos")]
  fn set_above_menu_bar(&self) -> anyhow::Result<()> {
    let ns_win =
      self.ns_window().context("Failed to get window handle.")? as id;

    unsafe {
      ns_win.setLevel_(
        ((NSMainMenuWindowLevel + 1) as u64)
          .try_into()
          .context("Failed to cast `NSMainMenuWindowLevel`.")?,
      );
    }

    Ok(())
  }

  #[cfg(target_os = "windows")]
  fn set_tool_window(&self, enable: bool) -> anyhow::Result<()> {
    let handle = self.hwnd().context("Failed to get window handle.")?;

    // Normally one would add the `WS_EX_TOOLWINDOW` style and remove the
    // `WS_EX_APPWINDOW` style to hide a window from the taskbar. Oddly
    // enough, this was causing the `WS_EX_APPWINDOW` style to be
    // preserved unless fully overwriting the extended window style.
    unsafe {
      match enable {
        true => SetWindowLongPtrW(
          handle,
          GWL_EXSTYLE,
          WS_EX_TOOLWINDOW.0 as isize,
        ),
        false => SetWindowLongPtrW(
          handle,
          GWL_EXSTYLE,
          WS_EX_APPWINDOW.0 as isize,
        ),
      }
    };

    Ok(())
  }
}
