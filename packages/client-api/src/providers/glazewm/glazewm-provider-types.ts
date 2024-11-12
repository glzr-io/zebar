import {
  TilingDirection,
  type BindingModeConfig,
  type Container,
  type Monitor,
  type RunCommandResponse,
  type Workspace,
} from 'glazewm';

import type { Provider } from '../create-base-provider';

export interface GlazeWmProviderConfig {
  type: 'glazewm';
}

export type GlazeWmProvider = Provider<
  GlazeWmProviderConfig,
  GlazeWmOutput
>;

export interface GlazeWmOutput {
  /**
   * Workspace displayed on the current monitor.
   */
  displayedWorkspace: Workspace;

  /**
   * Workspace that currently has focus (on any monitor).
   */
  focusedWorkspace: Workspace;

  /**
   * Workspaces on the current monitor.
   */
  currentWorkspaces: Workspace[];

  /**
   * Workspaces across all monitors.
   */
  allWorkspaces: Workspace[];

  /**
   * All monitors.
   */
  allMonitors: Monitor[];

  /**
   * All windows.
   */
  allWindows: Window[];

  /**
   * Monitor that currently has focus.
   */
  focusedMonitor: Monitor;

  /**
   * Monitor that is nearest to this Zebar widget.
   */
  currentMonitor: Monitor;

  /**
   * Container that currently has focus (on any monitor).
   */
  focusedContainer: Container;

  /**
   * Tiling direction of the focused container.
   */
  tilingDirection: TilingDirection;

  /**
   * Active binding modes;
   */
  bindingModes: BindingModeConfig[];

  /**
   * Invokes a WM command (e.g. `"focus --workspace 1"`).
   *
   * @param command WM command to run (e.g. `"focus --workspace 1"`).
   * @param subjectContainerId (optional) ID of container to use as subject.
   * If not provided, this defaults to the currently focused container.
   * @throws If command fails.
   */
  runCommand(
    command: string,
    subjectContainerId?: string,
  ): Promise<RunCommandResponse>;
}
