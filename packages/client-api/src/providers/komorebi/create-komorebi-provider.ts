import { createEffect, type Owner } from 'solid-js';
import { createStore } from 'solid-js/store';

import type { KomorebiProviderConfig } from '~/user-config';
import { createProviderListener } from '../create-provider-listener';
import { getMonitors } from '~/desktop';
import { getCoordinateDistance } from '~/utils';

export interface KomorebiProvider {
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
  monitors: KomorebiMonitor[];

  /**
   * Monitor that currently has focus.
   */
  focusedMonitor: KomorebiMonitor;

  /**
   * Monitor that is nearest to this Zebar window.
   */
  currentMonitor: KomorebiMonitor;
}

export interface KomorebiMonitor {
  id: number;
  name: string;
  deviceId: string;
  focusedWorkspaceIndex: number;
  size: KomorebiRect;
  workAreaOffset: number | null;
  workAreaSize: KomorebiRect;
  workspaces: KomorebiWorkspace[];
}

export interface KomorebiWorkspace {
  containerPadding: number;
  floatingWindows: KomorebiWindow[];
  latestLayout: KomorebiRect[];
  layout: KomorebiLayout;
  layoutFlip: KomorebiLayoutFlip | null;
  name: string;
  maximizedWindow: KomorebiWindow | null;
  monocleContainer: KomorebiContainer | null;
  tilingContainers: KomorebiContainer[];
  workspacePadding: number;
}

export interface KomorebiContainer {
  id: string;
  windows: KomorebiWindow[];
}

export interface KomorebiWindow {
  class: string;
  exe: string;
  hwnd: number;
  size: KomorebiRect;
  title: string;
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
  | 'rows';

export type KomorebiLayoutFlip = 'horizontal' | 'vertical';

export async function createKomorebiProvider(
  config: KomorebiProviderConfig,
  owner: Owner,
): Promise<KomorebiProvider> {
  const { currentMonitor } = await getMonitors();

  const providerListener = await createProviderListener<
    KomorebiProviderConfig,
    KomorebiProvider
  >(config, owner);

  const [komorebiVariables, setKomorebiVariables] = createStore(
    await getVariables(),
  );

  createEffect(async () => setKomorebiVariables(await getVariables()));

  async function getVariables() {
    const state = providerListener();
    const currentPosition = { x: currentMonitor!.x, y: currentMonitor!.y };

    // Get Komorebi monitor that corresponds to the window's monitor.
    const currentKomorebiMonitor = state.monitors.reduce((a, b) =>
      getCoordinateDistance(currentPosition, {
        x: a.workAreaSize.left,
        y: a.workAreaSize.top,
      }) <
      getCoordinateDistance(currentPosition, {
        x: b.workAreaSize.left,
        y: b.workAreaSize.top,
      })
        ? a
        : b,
    );

    const displayedWorkspace =
      currentKomorebiMonitor.workspaces[
        currentKomorebiMonitor.focusedWorkspaceIndex
      ]!;

    return {
      displayedWorkspace,
      focusedWorkspace: state.focusedWorkspace,
      currentWorkspaces: currentKomorebiMonitor.workspaces,
      allWorkspaces: state.allWorkspaces,
      monitors: state.monitors,
      focusedMonitor: state.focusedMonitor,
      currentMonitor: currentKomorebiMonitor,
    };
  }

  return {
    get displayedWorkspace() {
      return komorebiVariables.displayedWorkspace;
    },
    get focusedWorkspace() {
      return komorebiVariables.focusedWorkspace;
    },
    get currentWorkspaces() {
      return komorebiVariables.currentWorkspaces;
    },
    get allWorkspaces() {
      return komorebiVariables.allWorkspaces;
    },
    get monitors() {
      return komorebiVariables.monitors;
    },
    get focusedMonitor() {
      return komorebiVariables.focusedMonitor;
    },
    get currentMonitor() {
      return komorebiVariables.currentMonitor;
    },
  };
}
