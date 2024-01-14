import type { Owner } from 'solid-js';

import type { MemoryProviderConfig } from '~/user-config';
import { createProviderListener } from '../create-provider-listener';

export interface MemoryVariables {
  freeMemory: number;
  usedMemory: number;
  totalMemory: number;
  freeSwap: number;
  usedSwap: number;
  totalSwap: number;
}

export async function createMemoryProvider(
  config: MemoryProviderConfig,
  owner: Owner,
) {
  const memoryVariables = await createProviderListener<
    MemoryProviderConfig,
    MemoryVariables
  >(config, owner);

  return {
    get freeMemory() {
      return memoryVariables().freeMemory;
    },
    get usedMemory() {
      return memoryVariables().usedMemory;
    },
    get totalMemory() {
      return memoryVariables().totalMemory;
    },
    get freeSwap() {
      return memoryVariables().freeSwap;
    },
    get usedSwap() {
      return memoryVariables().usedSwap;
    },
    get totalSwap() {
      return memoryVariables().totalSwap;
    },
  };
}
