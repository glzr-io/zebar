import {
  onProviderEmit,
  listenProvider,
  unlistenProvider,
} from '~/desktop';
import { Deferred, simpleHash } from '~/utils';
import type { ProviderConfig } from './create-provider';

export interface ProviderListener<TVars> {
  firstValue: TVars;
  onChange: (callback: (val: TVars) => void) => void;
  unlisten: () => Promise<void>;
}

/**
 * Utility for listening to a provider of a given config type.
 */
export async function createProviderListener<TVars>(
  config: ProviderConfig,
): Promise<ProviderListener<TVars>> {
  const configHash = simpleHash(config);

  const firstValue = new Deferred<TVars>();
  const listeners: ((val: TVars) => void)[] = [];

  const unlistenEmit = await onProviderEmit<TVars>(configHash, val => {
    firstValue.resolve(val);
    listeners.forEach(listener => listener(val));
  });

  await listenProvider({
    configHash,
    config,
    trackedAccess: [],
  });

  return {
    firstValue: await firstValue.promise,
    onChange: callback => {
      listeners.push(callback);
    },
    unlisten: async () => {
      unlistenEmit();
      await unlistenProvider(configHash);
    },
  };
}
