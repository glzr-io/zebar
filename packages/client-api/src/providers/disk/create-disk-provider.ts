import { z } from 'zod';

import { createBaseProvider } from '../create-base-provider';
import { onProviderEmit } from '~/desktop';
import type {
  DiskOutput,
  DiskProvider,
  DiskProviderConfig,
} from './disk-provider-types';

const diskProviderConfigSchema = z.object({
  type: z.literal('disk'),
  refreshInterval: z.coerce.number().default(60 * 1000),
});

export function createDiskProvider(
  config: DiskProviderConfig,
): DiskProvider {
  const mergedConfig = diskProviderConfigSchema.parse(config);

  return createBaseProvider(mergedConfig, async queue => {
    return onProviderEmit<DiskOutput>(mergedConfig, ({ result }) => {
      if ('error' in result) {
        queue.error(result.error);
      } else {
        queue.output(result.output);
      }
    });
  });
}
