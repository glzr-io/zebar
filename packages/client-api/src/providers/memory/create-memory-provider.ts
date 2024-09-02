import type { Owner } from 'solid-js';
import { z } from 'zod';

import { createProviderListener } from '../create-provider-listener';

export interface MemoryProviderConfig {
  type: 'memory';

  /**
   * How often this provider refreshes in milliseconds.
   */
  refreshInterval?: number;
}

const MemoryProviderConfigSchema = z.object({
  type: z.literal('memory'),
  refreshInterval: z.coerce.number().default(5 * 1000),
});

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
  const mergedConfig = MemoryProviderConfigSchema.parse(config);

  const memoryVariables = await createProviderListener<
    MemoryProviderConfig,
    MemoryProvider
  >(mergedConfig, owner);

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
