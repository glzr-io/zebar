import {
  onProviderEmit,
  listenProvider,
  unlistenProvider,
} from '~/desktop';
import { simpleHash } from '~/utils';
import type { ProviderConfig } from './create-provider';

export interface ProviderListener<TVars> {
  firstValue: TVars;
  onChange: (callback: (val: TVars) => void) => void;
  unlisten: () => void;
}

/**
 * Utility for listening to a provider of a given config type.
 */
export async function createProviderListener<
  TConfig extends ProviderConfig,
  TVars,
>(config: TConfig): Promise<ProviderListener<TVars>> {
  const configHash = simpleHash(config);
  const listeners: ((val: TVars) => void)[] = [];

  const unlistenEmit = await onProviderEmit<TVars>(configHash, val => {
    listeners.forEach(listener => listener(val));
  });

  const firstValue = new Promise<TVars>(async resolve => {
    const unsubscribe = await onProviderEmit<TVars>(configHash, value => {
      unsubscribe();
      resolve(value);
    });
  });

  await listenProvider({
    configHash,
    config,
    trackedAccess: [],
  });

  return {
    firstValue: await firstValue,
    onChange: (callback: (val: TVars) => void) => {
      listeners.push(callback);
    },
    unlisten: async () => {
      unlistenEmit();
      await unlistenProvider(configHash);
    },
  };
}
