import { z } from 'zod';

import { getMonitors, onProviderEmit } from '~/desktop';
import { getCoordinateDistance } from '~/utils';
import { createBaseProvider } from '../create-base-provider';
import type {
  KomorebiProvider,
  KomorebiProviderConfig,
  KomorebiResponse,
} from './komorebi-provider-types';

const komorebiProviderConfigSchema = z.object({
  type: z.literal('komorebi'),
});

export function createKomorebiProvider(
  config: KomorebiProviderConfig,
): KomorebiProvider {
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
