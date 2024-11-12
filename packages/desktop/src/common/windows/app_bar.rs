use anyhow::bail;
use tauri::{PhysicalPosition, PhysicalSize};
use tracing::info;
use windows::Win32::{
  Foundation::{HWND, RECT},
  UI::Shell::{
    SHAppBarMessage, ABE_BOTTOM, ABE_LEFT, ABE_RIGHT, ABE_TOP, ABM_NEW,
    ABM_QUERYPOS, ABM_REMOVE, ABM_SETPOS, APPBARDATA,
  },
};

use crate::config::DockEdge;

pub fn create_app_bar(
  window_handle: isize,
  size: PhysicalSize<i32>,
  position: PhysicalPosition<i32>,
  edge: DockEdge,
) -> anyhow::Result<(PhysicalSize<i32>, PhysicalPosition<i32>)> {
  let rect = RECT {
    left: position.x,
    top: position.y,
    right: position.x + size.width,
    bottom: position.y + size.height,
  };

  let edge = match edge {
    DockEdge::Left => ABE_LEFT,
    DockEdge::Top => ABE_TOP,
    DockEdge::Right => ABE_RIGHT,
    DockEdge::Bottom => ABE_BOTTOM,
  };

  let mut data = APPBARDATA {
    cbSize: std::mem::size_of::<APPBARDATA>() as u32,
    hWnd: HWND(window_handle as _),
    uCallbackMessage: 0,
    uEdge: edge,
    rc: rect.clone(),
    ..Default::default()
  };

  if unsafe { SHAppBarMessage(ABM_NEW, &mut data) } == 0 {
    bail!("Failed to register new app bar.");
  }

  if unsafe { SHAppBarMessage(ABM_QUERYPOS, &mut data) } == 0 {
    bail!("Failed to query for app bar position.");
  }

  // Calculate the adjusted position directly from data.rc.
  let adjusted_position = PhysicalPosition::new(
    position.x + (data.rc.left - rect.left) + (data.rc.right - rect.right),
    position.y + (data.rc.top - rect.top) + (data.rc.bottom - rect.bottom),
  );

  // Update the rect with the adjusted position while keeping the original
  // size.
  data.rc = RECT {
    left: adjusted_position.x,
    top: adjusted_position.y,
    right: adjusted_position.x + size.width,
    bottom: adjusted_position.y + size.height,
  };

  // Set position for it to actually reserve the size and position.
  if unsafe { SHAppBarMessage(ABM_SETPOS, &mut data) } == 0 {
    bail!("Failed to set app bar position.");
  }

  info!("Successfully registered appbar with rect: {:?}", data.rc);

  Ok((size, adjusted_position))
}

/// Deallocate the app bar for given window handle.
///
/// Note that this does not error if handle is invalid.
pub fn remove_app_bar(handle: isize) -> anyhow::Result<()> {
  info!("Removing app bar for {:?}.", handle);

  let mut abd = APPBARDATA {
    cbSize: std::mem::size_of::<APPBARDATA>() as u32,
    hWnd: HWND(handle as _),
    uCallbackMessage: 0,
    ..Default::default()
  };

  match unsafe { SHAppBarMessage(ABM_REMOVE, &mut abd) } {
    0 => bail!("Failed to remove app bar."),
    _ => Ok(()),
  }
}
