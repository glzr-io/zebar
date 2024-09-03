import {
  listen,
  type Event,
  type UnlistenFn,
} from '@tauri-apps/api/event';
import type { ProviderConfig } from '~/providers';

import { createLogger, simpleHash } from '~/utils';
import { listenProvider, unlistenProvider } from './desktop-commands';

const logger = createLogger('desktop-events');

let listenPromise: Promise<UnlistenFn> | null = null;

let callbacks: {
  configHash: string;
  fn: (payload: Event<ProviderEmitEvent<any>>) => void;
}[] = [];

export interface ProviderEmitEvent<T = unknown> {
  configHash: string;
  variables: { data: T } | { error: string };
}

/**
 * Listen for provider data.
 */
export async function onProviderEmit<T = unknown>(
  config: ProviderConfig,
  callback: (event: ProviderEmitEvent<T>) => void,
): Promise<() => Promise<void>> {
  const configHash = simpleHash(config);

  registerEventCallback(configHash, callback);

  const unlisten = await (listenPromise ??
    (listenPromise = listenProviderEmit()));

  await listenProvider({
    configHash,
    config,
    trackedAccess: [],
  });

  return async () => {
    callbacks = callbacks.filter(
      callback => callback.configHash !== configHash,
    );

    await unlistenProvider(configHash);

    // Unlisten when there are no active callbacks.
    if (callbacks.length === 0) {
      unlisten();
      listenPromise = null;
    }
  };
}

/**
 * Add callback to invoke when a provider emits data.
 */
function registerEventCallback<T>(
  configHash: string,
  callback: (event: ProviderEmitEvent<T>) => void,
) {
  const wrappedCallback = (event: Event<ProviderEmitEvent<T>>) => {
    // Ignore provider emissions for different configs.
    if (event.payload.configHash !== configHash) {
      return;
    }

    logger.debug('Incoming provider emission:', event.payload);
    callback(event.payload);
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
