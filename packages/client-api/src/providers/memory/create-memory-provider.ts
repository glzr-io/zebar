import { z } from 'zod';

import { createBaseProvider } from '../create-base-provider';
import { onProviderEmit } from '~/desktop';
import type {
  MemoryOutput,
  MemoryProvider,
  MemoryProviderConfig,
} from './memory-provider-types';

const memoryProviderConfigSchema = z.object({
  type: z.literal('memory'),
  refreshInterval: z.coerce.number().default(5 * 1000),
});

export function createMemoryProvider(
  config: MemoryProviderConfig,
): MemoryProvider {
  const mergedConfig = memoryProviderConfigSchema.parse(config);

  return createBaseProvider(mergedConfig, async queue => {
    return onProviderEmit<MemoryOutput>(mergedConfig, ({ result }) => {
      if ('error' in result) {
        queue.error(result.error);
      } else {
        queue.output(result.output);
      }
    });
  });
}
