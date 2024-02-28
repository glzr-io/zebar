use komorebi_client::{Axis, DefaultLayout, Layout, Rect};
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct KomorebiVariables {
  pub all_monitors: Vec<KomorebiMonitor>,
  pub focused_monitor_index: usize,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct KomorebiMonitor {
  pub id: isize,
  pub device_id: Option<String>,
  pub focused_workspace_index: usize,
  pub name: String,
  pub size: Rect,
  pub work_area_offset: Option<Rect>,
  pub work_area_size: Rect,
  pub workspaces: Vec<KomorebiWorkspace>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct KomorebiWorkspace {
  pub container_padding: Option<i32>,
  pub floating_windows: Vec<KomorebiWindow>,
  pub focused_container_index: usize,
  pub latest_layout: Vec<Rect>,
  pub layout: KomorebiLayout,
  pub layout_flip: Option<KomorebiLayoutFlip>,
  pub maximized_window: Option<KomorebiWindow>,
  pub monocle_container: Option<KomorebiContainer>,
  pub name: Option<String>,
  pub tiling_containers: Vec<KomorebiContainer>,
  pub workspace_padding: Option<i32>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct KomorebiContainer {
  pub id: String,
  pub windows: Vec<KomorebiWindow>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct KomorebiWindow {
  pub class: Option<String>,
  pub exe: Option<String>,
  pub hwnd: u64,
  pub title: Option<String>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum KomorebiLayout {
  Bsp,
  VerticalStack,
  HorizontalStack,
  UltrawideVerticalStack,
  Rows,
  Grid,
  Custom,
}

impl From<Layout> for KomorebiLayout {
  fn from(layout: Layout) -> Self {
    match layout {
      Layout::Default(layout) => match layout {
        DefaultLayout::BSP => KomorebiLayout::Bsp,
        DefaultLayout::Columns => KomorebiLayout::Custom,
        DefaultLayout::Rows => KomorebiLayout::Rows,
        DefaultLayout::VerticalStack => KomorebiLayout::VerticalStack,
        DefaultLayout::HorizontalStack => KomorebiLayout::HorizontalStack,
        DefaultLayout::UltrawideVerticalStack => {
          KomorebiLayout::UltrawideVerticalStack
        }
        DefaultLayout::Grid => KomorebiLayout::Grid,
      },
      _ => KomorebiLayout::Custom,
    }
  }
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum KomorebiLayoutFlip {
  Horizontal,
  Vertical,
  HorizontalAndVertical,
}

impl From<Axis> for KomorebiLayoutFlip {
  fn from(axis: Axis) -> Self {
    match axis {
      Axis::Horizontal => KomorebiLayoutFlip::Horizontal,
      Axis::Vertical => KomorebiLayoutFlip::Vertical,
      Axis::HorizontalAndVertical => {
        KomorebiLayoutFlip::HorizontalAndVertical
      }
    }
  }
}
