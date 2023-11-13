import { createEffect } from 'solid-js';
import { createStore } from 'solid-js/store';

import { onProviderEmit, listenProvider, unlistenProvider } from '~/desktop';
import {
  MemoryProviderOptions,
  MemoryProviderOptionsSchema,
} from '~/user-config';
import { memoize, simpleHash } from '~/utils';

const DEFAULT = MemoryProviderOptionsSchema.parse({});

export interface MemoryVariables {
  isLoading: boolean;
  freeMemory: number;
  usedMemory: number;
  totalMemory: number;
  freeSwap: number;
  usedSwap: number;
  totalSwap: number;
}

export const createMemoryProvider = memoize(
  (options: MemoryProviderOptions = DEFAULT) => {
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
      const optionsHash = simpleHash(options);

      onProviderEmit<MemoryVariables>(optionsHash, payload =>
        setMemoryVariables({ ...payload, isLoading: false }),
      );

      await listenProvider({
        optionsHash,
        options,
        trackedAccess: [],
      });

      return () => unlistenProvider(optionsHash);
    });

    return {
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
  },
);
