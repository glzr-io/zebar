use windows::Win32::{
  Foundation::{HWND, RECT},
  UI::Shell::{
    SHAppBarMessage, ABE_BOTTOM, ABE_LEFT, ABE_RIGHT, ABE_TOP, ABM_NEW,
    ABM_REMOVE, ABM_SETPOS, APPBARDATA,
  },
};

use crate::config::WidgetEdge;

pub fn create_app_bar(
  hwnd: HWND,
  left: i32,
  top: i32,
  right: i32,
  bottom: i32,
  edge: WidgetEdge,
) {
  if hwnd.is_invalid() {
    tracing::trace!("Invalid hwnd passed to create_app_bar");
    return;
  }

  let rect = RECT {
    left,
    top,
    right,
    bottom,
  };

  let edge = match edge {
    WidgetEdge::Left => ABE_LEFT,
    WidgetEdge::Top => ABE_TOP,
    WidgetEdge::Right => ABE_RIGHT,
    WidgetEdge::Bottom => ABE_BOTTOM,
  };

  tracing::trace!(
    "Registering app bar for {:?} with edge: {:?} and rect: {:?}",
    hwnd,
    edge,
    rect
  );

  let mut data = APPBARDATA {
    cbSize: std::mem::size_of::<APPBARDATA>() as u32,
    hWnd: hwnd,
    uCallbackMessage: 0,
    uEdge: edge,
    rc: rect,
    ..Default::default()
  };

  unsafe { SHAppBarMessage(ABM_NEW, &mut data) };

  // Set position for it to actually reserve the size and position.
  unsafe { SHAppBarMessage(ABM_SETPOS, &mut data) };
}

pub fn remove_app_bar(handle: isize) {
  tracing::trace!("Removing app bar for {:?}.", handle);

  let mut abd = APPBARDATA {
    hWnd: HWND(handle as _),
    ..Default::default()
  };

  unsafe { SHAppBarMessage(ABM_REMOVE, &mut abd) };
}
