import { createEffect } from 'solid-js';
import { createStore } from 'solid-js/store';

import { onProviderEmit, listenProvider, unlistenProvider } from '~/desktop';
import { MemoryProviderConfig } from '~/user-config';
import { memoize, simpleHash } from '~/utils';

export interface MemoryVariables {
  isLoading: boolean;
  freeMemory: number;
  usedMemory: number;
  totalMemory: number;
  freeSwap: number;
  usedSwap: number;
  totalSwap: number;
}

export const createMemoryProvider = memoize((config: MemoryProviderConfig) => {
  const [memoryVariables, setMemoryVariables] = createStore<MemoryVariables>({
    isLoading: true,
    freeMemory: 0,
    usedMemory: 0,
    totalMemory: 0,
    freeSwap: 0,
    usedSwap: 0,
    totalSwap: 0,
  });

  createEffect(async () => {
    const configHash = simpleHash(config);

    onProviderEmit<MemoryVariables>(configHash, payload =>
      setMemoryVariables({ ...payload, isLoading: false }),
    );

    await listenProvider({
      configHash: configHash,
      config: config,
      trackedAccess: [],
    });

    return () => unlistenProvider(configHash);
  });

  return {
    get isLoading() {
      return memoryVariables.isLoading;
    },
    get freeMemory() {
      return memoryVariables.freeMemory;
    },
    get usedMemory() {
      return memoryVariables.usedMemory;
    },
    get totalMemory() {
      return memoryVariables.totalMemory;
    },
    get freeSwap() {
      return memoryVariables.freeSwap;
    },
    get usedSwap() {
      return memoryVariables.usedSwap;
    },
    get totalSwap() {
      return memoryVariables.totalSwap;
    },
  };
});
