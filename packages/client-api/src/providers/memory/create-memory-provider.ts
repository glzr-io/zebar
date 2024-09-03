import { z } from 'zod';

import {
  createBaseProvider,
  type Provider,
} from '../create-base-provider';
import { onProviderEmit } from '~/desktop';

export interface MemoryProviderConfig {
  type: 'memory';

  /**
   * How often this provider refreshes in milliseconds.
   */
  refreshInterval?: number;
}

const memoryProviderConfigSchema = z.object({
  type: z.literal('memory'),
  refreshInterval: z.coerce.number().default(5 * 1000),
});

export type MemoryProvider = Provider<MemoryProviderConfig, MemoryOutput>;

export interface MemoryOutput {
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
): Promise<MemoryProvider> {
  const mergedConfig = memoryProviderConfigSchema.parse(config);

  return createBaseProvider(mergedConfig, async queue => {
    return onProviderEmit<MemoryOutput>(mergedConfig, ({ variables }) => {
      if ('error' in variables) {
        queue.error(variables.error);
      } else {
        queue.value(variables.data);
      }
    });
  });
}
