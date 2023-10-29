import { listen, Event } from '@tauri-apps/api/event';

import { createLogger } from '../utils';

const logger = createLogger('desktop-events');

export interface ProviderEmitEvent<T = unknown> {
  configHash: string;
  data: T;
}

/**
 * Listen for provider data.
 */
export function onProviderEmit<T = unknown>(
  configHash: string,
  callback: (payload: T) => void,
) {
  logger.debug(`Listening to provider with config: ${configHash}.`);

  return listen('provider-emit', (event: Event<ProviderEmitEvent<T>>) => {
    const { payload } = event;

    if (payload.configHash === configHash) {
      callback(payload as T);
    }
  });
}
