use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KomorebiOutput {
  pub all_monitors: Vec<KomorebiMonitor>,
  pub focused_monitor_index: usize,
}

impl<'de> Deserialize<'de> for KomorebiOutput {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    #[derive(Deserialize)]
    struct Notification {
      state: State,
    }

    #[derive(Deserialize)]
    struct State {
      monitors: MonitorElements,
    }

    #[derive(Deserialize)]
    struct MonitorElements {
      elements: Vec<KomorebiMonitor>,
      focused: usize,
    }

    let notification = Notification::deserialize(deserializer)?;

    Ok(KomorebiOutput {
      all_monitors: notification.state.monitors.elements,
      focused_monitor_index: notification.state.monitors.focused,
    })
  }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KomorebiMonitor {
  pub id: isize,
  pub device_id: String,
  pub focused_workspace_index: usize,
  pub name: String,
  pub size: Rect,
  pub work_area_offset: Option<Rect>,
  pub work_area_size: Rect,
  pub workspaces: Vec<KomorebiWorkspace>,
}

impl<'de> Deserialize<'de> for KomorebiMonitor {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    #[derive(Deserialize)]
    struct Monitor {
      id: isize,
      device_id: String,
      name: String,
      size: Rect,
      work_area_offset: Option<Rect>,
      work_area_size: Rect,
      workspaces: WorkspaceElements,
    }

    #[derive(Deserialize)]
    struct WorkspaceElements {
      elements: Vec<KomorebiWorkspace>,
      focused: usize,
    }

    let monitor = Monitor::deserialize(deserializer)?;

    Ok(KomorebiMonitor {
      id: monitor.id,
      device_id: monitor.device_id,
      name: monitor.name,
      size: monitor.size,
      work_area_offset: monitor.work_area_offset,
      work_area_size: monitor.work_area_size,
      workspaces: monitor.workspaces.elements,
      focused_workspace_index: monitor.workspaces.focused,
    })
  }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
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

impl<'de> Deserialize<'de> for KomorebiWorkspace {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    #[derive(Deserialize)]
    struct Workspace {
      container_padding: Option<i32>,
      floating_windows: WindowElements,
      latest_layout: Vec<Rect>,
      layout: KomorebiLayout,
      layout_flip: Option<KomorebiLayoutFlip>,
      maximized_window: Option<KomorebiWindow>,
      monocle_container: Option<KomorebiContainer>,
      name: Option<String>,
      containers: ContainerElements,
      workspace_padding: Option<i32>,
    }

    #[derive(Deserialize)]
    struct ContainerElements {
      elements: Vec<KomorebiContainer>,
      focused: usize,
    }

    #[derive(Deserialize)]
    struct WindowElements {
      elements: Vec<KomorebiWindow>,
    }

    let workspace = Workspace::deserialize(deserializer)?;

    Ok(KomorebiWorkspace {
      container_padding: workspace.container_padding,
      floating_windows: workspace.floating_windows.elements,
      focused_container_index: workspace.containers.focused,
      latest_layout: workspace.latest_layout,
      layout: workspace.layout,
      layout_flip: workspace.layout_flip,
      maximized_window: workspace.maximized_window,
      monocle_container: workspace.monocle_container,
      name: workspace.name,
      tiling_containers: workspace.containers.elements,
      workspace_padding: workspace.workspace_padding,
    })
  }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KomorebiContainer {
  pub id: String,
  pub windows: Vec<KomorebiWindow>,
}

impl<'de> Deserialize<'de> for KomorebiContainer {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    #[derive(Deserialize)]
    struct Container {
      id: String,
      windows: WindowElements,
    }

    #[derive(Deserialize)]
    struct WindowElements {
      elements: Vec<KomorebiWindow>,
    }

    let container = Container::deserialize(deserializer)?;

    Ok(KomorebiContainer {
      id: container.id,
      windows: container.windows.elements,
    })
  }
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
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

impl<'de> Deserialize<'de> for KomorebiLayout {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    #[derive(Deserialize)]
    enum Layout {
      Default(DefaultLayout),
      #[serde(other)]
      Custom,
    }

    #[derive(Deserialize)]
    enum DefaultLayout {
      #[serde(rename = "BSP")]
      Bsp,
      VerticalStack,
      HorizontalStack,
      UltrawideVerticalStack,
      Rows,
      Grid,
      RightMainVerticalStack,
      #[serde(other)]
      Custom,
    }

    let layout = Layout::deserialize(deserializer)?;

    Ok(match layout {
      Layout::Default(layout) => match layout {
        DefaultLayout::Bsp => KomorebiLayout::Bsp,
        DefaultLayout::VerticalStack => KomorebiLayout::VerticalStack,
        DefaultLayout::HorizontalStack => KomorebiLayout::HorizontalStack,
        DefaultLayout::UltrawideVerticalStack => {
          KomorebiLayout::UltrawideVerticalStack
        }
        DefaultLayout::Rows => KomorebiLayout::Rows,
        DefaultLayout::Grid => KomorebiLayout::Grid,
        DefaultLayout::RightMainVerticalStack => {
          KomorebiLayout::RightMainVerticalStack
        }
        DefaultLayout::Custom => KomorebiLayout::Custom,
      },
      Layout::Custom => KomorebiLayout::Custom,
    })
  }
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
#[serde(rename_all(serialize = "snake_case", deserialize = "PascalCase"))]
pub enum KomorebiLayoutFlip {
  Horizontal,
  Vertical,
  HorizontalAndVertical,
}

#[derive(Debug, Deserialize, Clone, Serialize, Eq, PartialEq)]
pub struct Rect {
  pub left: i32,
  pub top: i32,
  pub right: i32,
  pub bottom: i32,
}
