import type { Provider } from '../create-base-provider';

export interface SwayProviderConfig {
  type: 'sway';
}

export type SwayProvider = Provider<
  SwayProviderConfig,
  SwayState
>;


export type SwayNodeLayout = 'SplitH' | 'SplitV' | 'Stacked' | 'Tabbed' | 'Output' | 'Dockarea' | 'None';
export type SwayOrientation = 'Vertical' | 'Horizontal' | 'None';
export interface Rect {
  x: number,
  y: number,
  width: number,
  height: number,
}
export interface SwayWorkspace {
  id: number,
  num: number,
  name: string,
  layout: SwayNodeLayout,
  visible: boolean,
  focused: boolean,
  urgent: boolean,
  representation?: string,
  orientation: SwayOrientation,
  rect: Rect,
  output: String,
  focus: number[],
}

export interface SwayOutput {
  name: string,
  rect: Rect,
}

export interface SwayResponse {
  /**
   * Workspaces across all monitors.
   */
  allWorkspaces: SwayWorkspace[];

  allOutputs: SwayOutput[];

  /**
   * Available binding modes;
   */
  bindingModes: string[];

  activeBindingMode: string;
}

export interface SwayState extends SwayResponse {
  /**
   * Workspaces on the current monitor.
   */
  currentWorkspaces: SwayWorkspace[];
  // /**
  //  * Workspace displayed on the current monitor.
  //  */
  // displayedWorkspace: Workspace;
  //
  // /**
  //  * Workspace that currently has focus (on any monitor).
  //  */
  // focusedWorkspace: Workspace;

  currentOutput: SwayOutput;

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
   * Invokes a WM command (e.g. `"focus --workspace 1"`).
   *
   * @param command WM command to run (e.g. `"focus --workspace 1"`).
   * @param `subjectContainerId (optional) ID of container to use as subject.
   * If not provided, this defaults to the currently focused container.
   * @throws If command fails.
   */
  runCommand(command: string): Promise<void>;
}
