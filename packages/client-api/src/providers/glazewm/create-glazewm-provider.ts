import type { Owner } from 'solid-js';
import { createStore } from 'solid-js/store';
import { GwmClient, GwmEventType, type Workspace } from 'glazewm';

import { getMonitors } from '~/desktop';
import type { GlazewmProviderConfig } from '~/user-config';
import { getCoordinateDistance } from '~/utils';

export async function createGlazewmProvider(
  _: GlazewmProviderConfig,
  __: Owner,
) {
  const { currentMonitor } = await getMonitors();
  const client = new GwmClient();

  const [glazewmVariables, setGlazewmVariables] = createStore({
    workspacesOnMonitor: [] as Workspace[],
    // TODO
    bindingMode: '',
  });

  client.onConnect(e => console.log('onOpen', e));
  client.onMessage(e => console.log('onMessage', e));
  client.onDisconnect(e => console.log('onClose', e));
  client.onError(e => console.log('onError', e));

  // Get initial workspaces.
  await refetch();

  await client.subscribeMany(
    [GwmEventType.WORKSPACE_ACTIVATED, GwmEventType.WORKSPACE_DEACTIVATED],
    refetch,
  );

  async function refetch() {
    const monitors = await client.getMonitors();
    const currentPosition = { x: currentMonitor!.x, y: currentMonitor!.y };

    // Get GlazeWM monitor that corresponds to the bar's monitor.
    const monitor = monitors.reduce((a, b) =>
      getCoordinateDistance(currentPosition, a) <
      getCoordinateDistance(currentPosition, b)
        ? a
        : b,
    );

    setGlazewmVariables({ workspacesOnMonitor: monitor.children });
  }

  return {
    get workspacesOnMonitor() {
      return glazewmVariables.workspacesOnMonitor;
    },
  };
}
