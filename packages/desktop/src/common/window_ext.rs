use anyhow::Context;
#[cfg(target_os = "macos")]
use cocoa::{
  appkit::{NSMainMenuWindowLevel, NSWindow},
  base::id,
};
#[cfg(target_os = "windows")]
use tauri::{PhysicalPosition, PhysicalSize};
use tauri::{Runtime, Window};
#[cfg(target_os = "windows")]
use windows::Win32::UI::WindowsAndMessaging::{
  SetWindowLongPtrW, GWL_EXSTYLE, WS_EX_APPWINDOW, WS_EX_TOOLWINDOW,
};

#[cfg(target_os = "windows")]
use crate::config::WidgetEdge;

pub trait WindowExt {
  #[cfg(target_os = "macos")]
  fn set_above_menu_bar(&self) -> anyhow::Result<()>;

  #[cfg(target_os = "windows")]
  fn set_tool_window(&self, enable: bool) -> anyhow::Result<()>;

  #[cfg(target_os = "windows")]
  fn allocate_app_bar(
    &self,
    size: PhysicalSize<i32>,
    position: PhysicalPosition<i32>,
    edge: WidgetEdge,
  ) -> anyhow::Result<(PhysicalSize<i32>, PhysicalPosition<i32>)>;

  #[cfg(target_os = "windows")]
  fn deallocate_app_bar(&self) -> anyhow::Result<()>;
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

  #[cfg(target_os = "windows")]
  fn allocate_app_bar(
    &self,
    size: PhysicalSize<i32>,
    position: PhysicalPosition<i32>,
    edge: WidgetEdge,
  ) -> anyhow::Result<(PhysicalSize<i32>, PhysicalPosition<i32>)> {
    use super::app_bar;

    let handle = self.hwnd().context("Failed to get window handle.")?;
    app_bar::create_app_bar(handle.0 as _, size, position, edge)
  }

  #[cfg(target_os = "windows")]
  fn deallocate_app_bar(&self) -> anyhow::Result<()> {
    use super::app_bar;

    let handle = self.hwnd().context("Failed to get window handle.")?;
    app_bar::remove_app_bar(handle.0 as _);

    Ok(())
  }
}
