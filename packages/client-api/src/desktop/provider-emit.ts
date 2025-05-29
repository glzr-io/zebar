import {
  listen,
  type Event,
  type UnlistenFn,
} from '@tauri-apps/api/event';
import type { ProviderConfig } from '~/providers';

import { createLogger, simpleHash } from '~/utils';
import { desktopCommands } from './desktop-commands';

let listenPromise: Promise<UnlistenFn> | null = null;

let callbacks: {
  configHash: string;
  fn: (payload: Event<ProviderEmitEvent<any>>) => void;
}[] = [];

export interface ProviderEmitEvent<T = unknown> {
  configHash: string;
  result: { output: T } | { error: string };
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

  await desktopCommands.listenProvider({
    configHash,
    config,
  });

  return async () => {
    callbacks = callbacks.filter(
      callback => callback.configHash !== configHash,
    );

    await desktopCommands.unlistenProvider(configHash);

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
