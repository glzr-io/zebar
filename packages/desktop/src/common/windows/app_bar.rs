use tauri::{PhysicalPosition, PhysicalSize};
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

  tracing::trace!(
    "Registering app bar for {:?} with edge: {:?} and rect: {:?}",
    window_handle,
    edge,
    rect
  );

  let mut data = APPBARDATA {
    cbSize: std::mem::size_of::<APPBARDATA>() as u32,
    hWnd: HWND(window_handle as _),
    uCallbackMessage: 0,
    uEdge: edge,
    rc: rect.clone(),
    ..Default::default()
  };
  tracing::info!("before SHAppBarMessage: {:?}", data.rc);

  let res = unsafe { SHAppBarMessage(ABM_NEW, &mut data) };
  tracing::info!("SHAppBarMessage(ABM_NEW) returned: {:?}", res);
  let res = unsafe { SHAppBarMessage(ABM_QUERYPOS, &mut data) };
  tracing::info!("SHAppBarMessage(ABM_QUERYPOS) returned: {:?}", res);

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
  let res = unsafe { SHAppBarMessage(ABM_SETPOS, &mut data) };
  tracing::info!("SHAppBarMessage(ABM_SETPOS) returned: {:?}", res);
  tracing::info!("after SHAppBarMessage: {:?}", data.rc);

  Ok((size, adjusted_position))
}

pub fn remove_app_bar(handle: isize) -> anyhow::Result<()> {
  tracing::trace!("Removing app bar for {:?}.", handle);

  let mut abd = APPBARDATA {
    hWnd: HWND(handle as _),
    ..Default::default()
  };

  unsafe { SHAppBarMessage(ABM_REMOVE, &mut abd) };

  Ok(())
}
