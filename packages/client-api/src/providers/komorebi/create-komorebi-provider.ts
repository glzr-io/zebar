import { z } from 'zod';

import { getMonitors, onProviderEmit } from '~/desktop';
import { getCoordinateDistance } from '~/utils';
import {
  createBaseProvider,
  type Provider,
} from '../create-base-provider';

export interface KomorebiProviderConfig {
  type: 'komorebi';
}

const komorebiProviderConfigSchema = z.object({
  type: z.literal('komorebi'),
});

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

interface KomorebiResponse {
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

export async function createKomorebiProvider(
  config: KomorebiProviderConfig,
): Promise<KomorebiProvider> {
  const mergedConfig = komorebiProviderConfigSchema.parse(config);

  // TODO: Update state when monitors change.
  return createBaseProvider(mergedConfig, async queue => {
    const monitors = await getMonitors();

    async function getUpdatedState(res: KomorebiResponse) {
      const currentPosition = {
        x: monitors.currentMonitor!.x,
        y: monitors.currentMonitor!.y,
      };

      // Get Komorebi monitor that corresponds to the Zebar window's monitor.
      const currentKomorebiMonitor = res.allMonitors.reduce((a, b) =>
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

      const displayedKomorebiWorkspace =
        currentKomorebiMonitor.workspaces[
          currentKomorebiMonitor.focusedWorkspaceIndex
        ]!;

      const allKomorebiWorkspaces = res.allMonitors.flatMap(
        monitor => monitor.workspaces,
      );

      const focusedKomorebiMonitor =
        res.allMonitors[res.focusedMonitorIndex]!;

      const focusedKomorebiWorkspace =
        focusedKomorebiMonitor.workspaces[
          focusedKomorebiMonitor.focusedWorkspaceIndex
        ]!;

      return {
        displayedWorkspace: displayedKomorebiWorkspace,
        focusedWorkspace: focusedKomorebiWorkspace,
        currentWorkspaces: currentKomorebiMonitor.workspaces,
        allWorkspaces: allKomorebiWorkspaces,
        focusedMonitor: focusedKomorebiMonitor,
        currentMonitor: currentKomorebiMonitor,
        allMonitors: res.allMonitors,
      };
    }

    return onProviderEmit<KomorebiResponse>(
      mergedConfig,
      async ({ result }) => {
        if ('error' in result) {
          queue.error(result.error);
        } else {
          const updatedState = await getUpdatedState(result.output);
          queue.output(updatedState);
        }
      },
    );
  });
}
