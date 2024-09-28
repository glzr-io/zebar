import { z } from 'zod';

import { createBaseProvider } from '../create-base-provider';
import { onProviderEmit } from '~/desktop';
import type {
  CpuOutput,
  CpuProvider,
  CpuProviderConfig,
} from './cpu-provider-types';

const cpuProviderConfigSchema = z.object({
  type: z.literal('cpu'),
  refreshInterval: z.coerce.number().default(5 * 1000),
});

export function createCpuProvider(config: CpuProviderConfig): CpuProvider {
  const mergedConfig = cpuProviderConfigSchema.parse(config);

  return createBaseProvider(mergedConfig, async queue => {
    return onProviderEmit<CpuOutput>(mergedConfig, ({ result }) => {
      if ('error' in result) {
        queue.error(result.error);
      } else {
        queue.output(result.output);
      }
    });
  });
}
