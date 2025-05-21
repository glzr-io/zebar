use anyhow::Context;
use tauri::{PhysicalPosition, PhysicalSize, Runtime, Window};
use windows::Win32::UI::WindowsAndMessaging::{
  SetWindowLongPtrW, GWL_EXSTYLE, WS_EX_APPWINDOW, WS_EX_TOOLWINDOW,
};

use super::app_bar;
use crate::widget_pack::DockEdge;

pub trait WindowExtWindows {
  fn set_tool_window(&self, enable: bool) -> anyhow::Result<()>;

  fn allocate_app_bar(
    &self,
    size: PhysicalSize<i32>,
    position: PhysicalPosition<i32>,
    edge: DockEdge,
  ) -> anyhow::Result<(PhysicalSize<i32>, PhysicalPosition<i32>)>;

  fn deallocate_app_bar(&self) -> anyhow::Result<()>;
}

impl<R: Runtime> WindowExtWindows for Window<R> {
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

  fn allocate_app_bar(
    &self,
    size: PhysicalSize<i32>,
    position: PhysicalPosition<i32>,
    edge: DockEdge,
  ) -> anyhow::Result<(PhysicalSize<i32>, PhysicalPosition<i32>)> {
    let handle = self.hwnd().context("Failed to get window handle.")?;
    app_bar::create_app_bar(handle.0 as _, size, position, edge)
  }

  fn deallocate_app_bar(&self) -> anyhow::Result<()> {
    let handle = self.hwnd().context("Failed to get window handle.")?;
    app_bar::remove_app_bar(handle.0 as _)
  }
}
