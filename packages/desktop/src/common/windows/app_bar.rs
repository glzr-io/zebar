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

  info!("Creating app bar with initial rect: {:?}", rect);

  let mut data = APPBARDATA {
    cbSize: std::mem::size_of::<APPBARDATA>() as u32,
    hWnd: HWND(window_handle as _),
    uCallbackMessage: 0,
    uEdge: match edge {
      DockEdge::Left => ABE_LEFT,
      DockEdge::Top => ABE_TOP,
      DockEdge::Right => ABE_RIGHT,
      DockEdge::Bottom => ABE_BOTTOM,
    },
    rc: rect.clone(),
    ..Default::default()
  };

  if unsafe { SHAppBarMessage(ABM_NEW, &mut data) } == 0 {
    bail!("Failed to register new app bar.");
  }

  // TODO: Ideally we should respond to incoming `ABN_POSCHANGED` messages.
  // Not responding to these messages causes space to be continually
  // reserved on hot-reloads in development. This is blocked by #11650.
  // Ref: https://github.com/tauri-apps/tauri/issues/11650.

  // Query to get the adjusted position. This only adjusts the edges of the
  // rect that have an appbar on them.
  // e.g. { left: 0, top: 0, right: 1920, bottom: 40 }
  // -> { left: 0, top: 80, right: 1920, bottom: 40 } (top edge adjusted)
  if unsafe { SHAppBarMessage(ABM_QUERYPOS, &mut data) } == 0 {
    bail!("Failed to query for app bar position.");
  }

  let adjusted_position = PhysicalPosition::new(data.rc.left, data.rc.top);

  let width_delta = match edge {
    DockEdge::Left => rect.right - data.rc.right,
    DockEdge::Right => rect.left - data.rc.left,
    _ => (rect.right - data.rc.right) - (rect.left - data.rc.left),
  };

  let height_delta = match edge {
    DockEdge::Top => rect.bottom - data.rc.bottom,
    DockEdge::Bottom => rect.top - data.rc.top,
    _ => (rect.bottom - data.rc.bottom) - (rect.top - data.rc.top),
  };

  // Size has changed if the edge that is not being docked has been
  // adjusted by ABM_QUERYPOS. For example, if the top edge is docked, then
  // diffs in the left and right edges are the size changes.
  let adjusted_size = PhysicalSize::new(
    size.width - width_delta,
    size.height - height_delta,
  );

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

  Ok((adjusted_size, adjusted_position))
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
