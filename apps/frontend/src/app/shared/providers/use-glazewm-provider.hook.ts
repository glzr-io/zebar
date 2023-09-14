import { createEffect, createResource } from 'solid-js';
import { GwmClient, GwmEventType } from 'glazewm';

import { memoize } from '../utils';
import { useLogger } from '../logging';

export const useGlazeWmProvider = memoize(() => {
  const logger = useLogger('useGlazeWm');

  const client = new GwmClient();

  client.onConnect(e => console.log('onOpen', e));
  client.onMessage(e => console.log('onMessage', e));
  client.onDisconnect(e => console.log('onClose', e));
  client.onError(e => console.log('onError', e));
  client.getMonitors().then(e => console.log('>>>>', e));

  const [workspaces, { refetch }] = createResource(() =>
    client.getWorkspaces(),
  );

  client.subscribe(GwmEventType.WORKSPACE_ACTIVATED, () => refetch());

  createEffect(() => console.info('workspaces changed', workspaces()));

  return {
    workspaces,
  };
});
