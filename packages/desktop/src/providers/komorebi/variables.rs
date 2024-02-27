use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct KomorebiVariables {
  pub monitors: Vec<KomorebiMonitor>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct KomorebiMonitor {
  pub id: isize,
  pub name: String,
  pub device_id: Option<String>,
  pub size: KomorebiRect,
  pub work_area_offset: Option<KomorebiRect>,
  pub work_area_size: KomorebiRect,
  pub workspaces: Vec<KomorebiWorkspace>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct KomorebiWorkspace {
  pub container_padding: u64,
  pub floating_windows: Vec<KomorebiWindow>,
  pub latest_layout: Vec<KomorebiRect>,
  pub layout: KomorebiLayout,
  pub layout_flip: Option<KomorebiLayoutFlip>,
  pub name: String,
  pub maximized_window: Option<KomorebiWindow>,
  pub monocle_container: Option<KomorebiWindow>,
  pub tiling_containers: Vec<KomorebiWindow>,
  pub workspace_padding: u64,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct KomorebiContainer {
  pub class: String,
  pub exe: String,
  pub hwnd: u64,
  pub size: KomorebiRect,
  pub title: String,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct KomorebiWindow {
  pub class: String,
  pub exe: String,
  pub hwnd: u64,
  pub size: KomorebiRect,
  pub title: String,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct KomorebiRect {
  pub left: u64,
  pub top: u64,
  pub right: u64,
  pub bottom: u64,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum KomorebiLayout {
  Bsp,
  VerticalStack,
  HorizontalStack,
  UltrawideVerticalStack,
  Rows,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum KomorebiLayoutFlip {
  Horizontal,
  Vertical,
}
