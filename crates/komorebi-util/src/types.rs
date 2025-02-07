use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KomorebiOutput {
  pub all_monitors: Vec<KomorebiMonitor>,
  pub focused_monitor_index: usize,
}

impl From<&komorebi_client::State> for KomorebiOutput {
  fn from(state: &komorebi_client::State) -> Self {
    KomorebiOutput {
      all_monitors: state
        .monitors
        .elements()
        .iter()
        .map(Into::into)
        .collect(),
      focused_monitor_index: state.monitors.focused_idx(),
    }
  }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KomorebiMonitor {
  pub id: isize,
  pub device_id: String,
  pub focused_workspace_index: usize,
  pub name: String,
  pub size: komorebi_client::Rect,
  pub work_area_offset: Option<komorebi_client::Rect>,
  pub work_area_size: komorebi_client::Rect,
  pub workspaces: Vec<KomorebiWorkspace>,
}

impl From<&komorebi_client::Monitor> for KomorebiMonitor {
  fn from(monitor: &komorebi_client::Monitor) -> Self {
    KomorebiMonitor {
      id: monitor.id(),
      name: monitor.name().to_string(),
      device_id: monitor.device_id().clone(),
      focused_workspace_index: monitor.focused_workspace_idx(),
      size: *monitor.size(),
      work_area_size: *monitor.work_area_size(),
      work_area_offset: monitor.work_area_offset(),
      workspaces: monitor.workspaces().iter().map(Into::into).collect(),
    }
  }
}

impl From<&komorebi_client::Container> for KomorebiContainer {
  fn from(container: &komorebi_client::Container) -> Self {
    KomorebiContainer {
      id: container.id().to_string(),
      windows: container.windows().iter().map(Into::into).collect(),
    }
  }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KomorebiWorkspace {
  pub container_padding: Option<i32>,
  pub floating_windows: Vec<KomorebiWindow>,
  pub focused_container_index: usize,
  pub latest_layout: Vec<komorebi_client::Rect>,
  pub layout: KomorebiLayout,
  pub layout_flip: Option<KomorebiLayoutFlip>,
  pub maximized_window: Option<KomorebiWindow>,
  pub monocle_container: Option<KomorebiContainer>,
  pub name: Option<String>,
  pub tiling_containers: Vec<KomorebiContainer>,
  pub workspace_padding: Option<i32>,
}

impl From<&komorebi_client::Workspace> for KomorebiWorkspace {
  fn from(workspace: &komorebi_client::Workspace) -> Self {
    KomorebiWorkspace {
      container_padding: workspace.container_padding(),
      floating_windows: workspace
        .floating_windows()
        .iter()
        .map(Into::into)
        .collect(),
      focused_container_index: workspace.focused_container_idx(),
      latest_layout: (*workspace.latest_layout()).clone(),
      layout: (*workspace.layout()).clone().into(),
      layout_flip: workspace.layout_flip().map(Into::into),
      name: workspace.name().clone(),
      maximized_window: workspace.maximized_window().map(|w| (&w).into()),
      monocle_container: workspace
        .monocle_container()
        .as_ref()
        .map(Into::into),
      tiling_containers: workspace
        .containers()
        .iter()
        .map(Into::into)
        .collect(),
      workspace_padding: workspace.workspace_padding(),
    }
  }
}

impl From<&komorebi_client::Window> for KomorebiWindow {
  fn from(window: &komorebi_client::Window) -> Self {
    KomorebiWindow {
      class: window.class().ok(),
      exe: window.exe().ok(),
      hwnd: window.hwnd().0 as u64,
      title: window.title().ok(),
    }
  }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KomorebiContainer {
  pub id: String,
  pub windows: Vec<KomorebiWindow>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KomorebiWindow {
  pub class: Option<String>,
  pub exe: Option<String>,
  pub hwnd: u64,
  pub title: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum KomorebiLayout {
  Bsp,
  VerticalStack,
  HorizontalStack,
  UltrawideVerticalStack,
  Rows,
  Grid,
  RightMainVerticalStack,
  Custom,
}

impl From<komorebi_client::Layout> for KomorebiLayout {
  fn from(layout: komorebi_client::Layout) -> Self {
    match layout {
      komorebi_client::Layout::Default(layout) => match layout {
        komorebi_client::DefaultLayout::BSP => KomorebiLayout::Bsp,
        komorebi_client::DefaultLayout::Columns => KomorebiLayout::Custom,
        komorebi_client::DefaultLayout::Rows => KomorebiLayout::Rows,
        komorebi_client::DefaultLayout::VerticalStack => {
          KomorebiLayout::VerticalStack
        }
        komorebi_client::DefaultLayout::HorizontalStack => {
          KomorebiLayout::HorizontalStack
        }
        komorebi_client::DefaultLayout::UltrawideVerticalStack => {
          KomorebiLayout::UltrawideVerticalStack
        }
        komorebi_client::DefaultLayout::Grid => KomorebiLayout::Grid,
        komorebi_client::DefaultLayout::RightMainVerticalStack => {
          KomorebiLayout::RightMainVerticalStack
        }
      },
      _ => KomorebiLayout::Custom,
    }
  }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum KomorebiLayoutFlip {
  Horizontal,
  Vertical,
  HorizontalAndVertical,
}

impl From<komorebi_client::Axis> for KomorebiLayoutFlip {
  fn from(axis: komorebi_client::Axis) -> Self {
    match axis {
      komorebi_client::Axis::Horizontal => KomorebiLayoutFlip::Horizontal,
      komorebi_client::Axis::Vertical => KomorebiLayoutFlip::Vertical,
      komorebi_client::Axis::HorizontalAndVertical => {
        KomorebiLayoutFlip::HorizontalAndVertical
      }
    }
  }
}
