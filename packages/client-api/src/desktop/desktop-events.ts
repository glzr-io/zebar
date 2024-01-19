import {
  listen,
  type Event,
  type UnlistenFn,
} from '@tauri-apps/api/event';

export interface ProviderEmitEvent<T = unknown> {
  configHash: string;
  variables: { data: T } | { error: string };
}

/**
 * Listen for provider data.
 */
export function onProviderEmit<T = unknown>(
  configHash: string,
  callback: (payload: T) => void,
): Promise<UnlistenFn> {
  return listen('provider-emit', (event: Event<ProviderEmitEvent<T>>) => {
    // Ignore provider emissions for different configs.
    if (event.payload.configHash !== configHash) {
      return;
    }

    const { variables } = event.payload;

    if ('error' in variables) {
      throw new Error(variables.error);
    }

    callback(variables.data as T);
  });
}
