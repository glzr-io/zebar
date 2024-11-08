use tauri::{PhysicalPosition, PhysicalSize};
use windows::Win32::{
  Foundation::{HWND, RECT},
  UI::Shell::{
    SHAppBarMessage, ABE_BOTTOM, ABE_LEFT, ABE_RIGHT, ABE_TOP, ABM_NEW,
    ABM_QUERYPOS, ABM_REMOVE, ABM_SETPOS, APPBARDATA,
  },
};

use crate::config::WidgetEdge;

pub fn create_app_bar(
  window_handle: isize,
  size: PhysicalSize<i32>,
  position: PhysicalPosition<i32>,
  edge: WidgetEdge,
) -> anyhow::Result<(PhysicalSize<i32>, PhysicalPosition<i32>)> {
  let rect = RECT {
    left: position.x,
    top: position.y,
    right: position.x + size.width,
    bottom: position.y + size.height,
  };

  let edge = match edge {
    WidgetEdge::Left => ABE_LEFT,
    WidgetEdge::Top => ABE_TOP,
    WidgetEdge::Right => ABE_RIGHT,
    WidgetEdge::Bottom => ABE_BOTTOM,
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

  let position = PhysicalPosition::new(
    position.x + (data.rc.left - rect.left),
    position.y + (data.rc.top - rect.top),
  );

  // Set position for it to actually reserve the size and position.
  let res = unsafe { SHAppBarMessage(ABM_SETPOS, &mut data) };
  tracing::info!("SHAppBarMessage(ABM_SETPOS) returned: {:?}", res);
  tracing::info!("after SHAppBarMessage: {:?}", data.rc);
}

pub fn remove_app_bar(handle: isize) {
  tracing::trace!("Removing app bar for {:?}.", handle);

  let mut abd = APPBARDATA {
    hWnd: HWND(handle as _),
    ..Default::default()
  };

  unsafe { SHAppBarMessage(ABM_REMOVE, &mut abd) };
}
