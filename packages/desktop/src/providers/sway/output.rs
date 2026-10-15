use serde::Serialize;
use swayipc_async::{Output, Workspace};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SwayOutput {
  // /**
  //  * Workspace displayed on the current monitor.
  //  */
  // displayedWorkspace: Workspace;
  //
  // /**
  //  * Workspace that currently has focus (on any monitor).
  //  */
  // focusedWorkspace: Workspace;
  /**
   * Workspaces on the current monitor.
   */
  // currentWorkspaces: Vec<Workspace>,

  /**
   * Workspaces across all monitors.
   */
  pub all_workspaces: Vec<Workspace>,

  /**
   * All monitors.
   */
  pub all_outputs: Vec<Output>,
  //
  // /**
  //  * All windows.
  //  */
  // allWindows: Window[];
  //
  // /**
  //  * Monitor that currently has focus.
  //  */
  // focusedMonitor: Monitor;
  //
  // /**
  //  * Monitor that is nearest to this Zebar widget.
  //  */
  // currentMonitor: Monitor;
  //
  // /**
  //  * Container that currently has focus (on any monitor).
  //  */
  // focusedContainer: Container;
  //
  // /**
  //  * Tiling direction of the focused container.
  //  */
  // tilingDirection: TilingDirection;
  /**
   * Available binding modes
   */
  pub binding_modes: Vec<String>,

  pub active_binding_mode: Option<String>,
}

pub fn workspace_eq(a: &Workspace, b: &Workspace) -> bool {
  a.id == b.id
    && a.num == b.num
    && a.name == b.name
    && a.layout == b.layout
    && a.visible == b.visible
    && a.focused == b.focused
    && a.urgent == b.urgent
    && a.representation == b.representation
    && a.rect == b.rect
    && a.output == b.output
    && a.focus == b.focus
}

// the swa ipc doesn't provide, which we can deal with later
impl PartialEq for SwayOutput {
  fn eq(&self, other: &Self) -> bool {
    self
      .all_workspaces
      .iter()
      .zip(other.all_workspaces.iter())
      .all(|(a, b)| workspace_eq(a, b))
      && self.binding_modes == other.binding_modes
      && self.active_binding_mode == other.active_binding_mode
  }

  fn ne(&self, other: &Self) -> bool {
    !self.eq(other)
  }
}
