import type { Owner } from 'solid-js';

import { createProviderListener } from '../create-provider-listener';

export interface MemoryProviderConfig {
  type: 'memory';

  /**
   * How often this provider refreshes in milliseconds.
   */
  refreshInterval?: number;
}

export interface MemoryProvider {
  usage: number;
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
    MemoryProvider
  >(config, owner);

  return {
    get usage() {
      return memoryVariables().usage;
    },
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
