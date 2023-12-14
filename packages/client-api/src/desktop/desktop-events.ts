import { listen, Event } from '@tauri-apps/api/event';

export interface ProviderEmitEvent<T = unknown> {
  configHash: string;
  variables: T;
}

/**
 * Listen for provider data.
 */
export function onProviderEmit<T = unknown>(
  configHash: string,
  callback: (payload: T) => void,
) {
  return listen('provider-emit', (event: Event<ProviderEmitEvent<T>>) => {
    if (event.payload.configHash === configHash) {
      callback(event.payload.variables as T);
    }
  });
}
