import {
  listen,
  type Event,
  type UnlistenFn,
} from '@tauri-apps/api/event';

import { createLogger } from '~/utils';

const logger = createLogger('desktop-events');

export interface ProviderEmitEvent<T = unknown> {
  configHash: string;
  variables: { data: T } | { error: string };
}

let listenPromise: Promise<UnlistenFn> | null = null;

let callbacks: {
  configHash: string;
  fn: (payload: Event<ProviderEmitEvent<any>>) => void;
}[] = [];

/**
 * Listen for provider data.
 */
export async function onProviderEmit<T = unknown>(
  configHash: string,
  callback: (payload: T) => void,
): Promise<UnlistenFn> {
  registerEventCallback(configHash, callback);

  const unlisten = await (listenPromise ??
    (listenPromise = listenProviderEmit()));

  // Unlisten when there are no active callbacks.
  return () => {
    callbacks = callbacks.filter(
      callback => callback.configHash !== configHash,
    );

    if (callbacks.length === 0) {
      unlisten();
    }
  };
}

/**
 * Add callback to invoke when a provider emits data.
 */
function registerEventCallback<T>(
  configHash: string,
  callback: (payload: T) => void,
) {
  const wrappedCallback = (event: Event<ProviderEmitEvent<T>>) => {
    // Ignore provider emissions for different configs.
    if (event.payload.configHash !== configHash) {
      return;
    }

    const { variables } = event.payload;

    if ('error' in variables) {
      logger.error('Incoming provider error:', variables.error);
      throw new Error(variables.error);
    }

    logger.debug('Incoming provider variables:', variables.data);
    callback(variables.data as T);
  };

  callbacks.push({ configHash, fn: wrappedCallback });
}

/**
 * Create listener for provider emissions.
 *
 * Only one Tauri event listener is needed to listen to all providers.
 */
async function listenProviderEmit(): Promise<UnlistenFn> {
  return listen('provider-emit', (event: Event<ProviderEmitEvent>) => {
    callbacks.forEach(callback => {
      if (event.payload.configHash === callback.configHash) {
        callback.fn(event);
      }
    });
  });
}
