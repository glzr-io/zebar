import { z } from 'zod';

import { createBaseProvider } from '../create-base-provider';
import { onProviderEmit } from '~/desktop';
import type {
  GpuOutput,
  GpuProvider,
  GpuProviderConfig,
} from './gpu-provider-types';

const gpuProviderConfigSchema = z.object({
  type: z.literal('gpu'),
  refreshInterval: z.coerce.number().default(5 * 1000),
});

export function createGpuProvider(config: GpuProviderConfig): GpuProvider {
  const mergedConfig = gpuProviderConfigSchema.parse(config);

  return createBaseProvider(mergedConfig, async queue => {
    return onProviderEmit<GpuOutput>(mergedConfig, ({ result }) => {
      if ('error' in result) {
        queue.error(result.error);
      } else {
        queue.output(result.output);
      }
    });
  });
}
