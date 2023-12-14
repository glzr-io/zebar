import { Owner } from 'solid-js';
import { createStore } from 'solid-js/store';
import { GwmClient, GwmEventType, Workspace } from 'glazewm';

import { GlazewmProviderConfig } from '~/user-config';
import { getMonitorPosition } from '~/desktop';

export async function createGlazewmProvider(
  _: GlazewmProviderConfig,
  __: Owner,
) {
  const client = new GwmClient();

  const [glazewmVariables, setGlazewmVariables] = createStore({
    workspaces: [] as Workspace[],
    binding_mode: '',
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
    const currentPosition = await getMonitorPosition();
    const monitors = await client.getMonitors();

    // Get GlazeWM monitor that corresponds to the bar's monitor.
    const monitor = monitors.reduce((a, b) =>
      getDistance(currentPosition, a) < getDistance(currentPosition, b) ? a : b,
    );

    setGlazewmVariables({ workspaces: monitor.children });
  }

  function getDistance(
    pointA: { x: number; y: number },
    pointB: { x: number; y: number },
  ) {
    return Math.sqrt(
      Math.pow(pointB.x - pointA.x, 2) + Math.pow(pointB.y - pointA.y, 2),
    );
  }

  return {
    get workspaces() {
      return glazewmVariables.workspaces;
    },
    get binding_mode() {
      return glazewmVariables.binding_mode;
    },
    focus_workspace: () => {},
  };
}
