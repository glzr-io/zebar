import type { Provider } from '../create-base-provider';

export interface KomorebiProviderConfig {
  type: 'komorebi';
}

export type KomorebiProvider = Provider<
  KomorebiProviderConfig,
  KomorebiOutput
>;

export interface KomorebiOutput {
  /**
   * Workspace displayed on the current monitor.
   */
  displayedWorkspace: KomorebiWorkspace;

  /**
   * Workspace that currently has focus (on any monitor).
   */
  focusedWorkspace: KomorebiWorkspace;

  /**
   * Workspaces on the current monitor.
   */
  currentWorkspaces: KomorebiWorkspace[];

  /**
   * Workspaces across all monitors.
   */
  allWorkspaces: KomorebiWorkspace[];

  /**
   * All monitors.
   */
  allMonitors: KomorebiMonitor[];

  /**
   * Monitor that currently has focus.
   */
  focusedMonitor: KomorebiMonitor;

  /**
   * Monitor that is nearest to this Zebar widget.
   */
  currentMonitor: KomorebiMonitor;
}

export interface KomorebiResponse {
  allMonitors: KomorebiMonitor[];
  focusedMonitorIndex: number;
}

export interface KomorebiMonitor {
  id: number;
  deviceId: string;
  focusedWorkspaceIndex: number;
  name: string;
  size: KomorebiRect;
  workAreaOffset: number | null;
  workAreaSize: KomorebiRect;
  workspaces: KomorebiWorkspace[];
}

export interface KomorebiWorkspace {
  containerPadding: number | null;
  floatingWindows: KomorebiWindow[];
  focusedContainerIndex: number;
  latestLayout: KomorebiRect[];
  layout: KomorebiLayout;
  layoutFlip: KomorebiLayoutFlip | null;
  maximizedWindow: KomorebiWindow | null;
  monocleContainer: KomorebiContainer | null;
  name: string | null;
  tilingContainers: KomorebiContainer[];
  workspacePadding: number | null;
}

export interface KomorebiContainer {
  id: string;
  windows: KomorebiWindow[];
}

export interface KomorebiWindow {
  class: string | null;
  exe: string | null;
  hwnd: number;
  title: string | null;
}

export interface KomorebiRect {
  left: number;
  top: number;
  right: number;
  bottom: number;
}

export type KomorebiLayout =
  | 'bsp'
  | 'vertical_stack'
  | 'horizontal_stack'
  | 'ultrawide_vertical_stack'
  | 'rows'
  | 'grid'
  | 'right_main_vertical_stack'
  | 'custom';

export type KomorebiLayoutFlip =
  | 'horizontal'
  | 'vertical'
  | 'horizontal_and_vertical';
