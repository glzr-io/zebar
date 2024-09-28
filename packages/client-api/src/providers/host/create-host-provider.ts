import { z } from 'zod';

import { createBaseProvider } from '../create-base-provider';
import { onProviderEmit } from '~/desktop';
import type {
  HostOutput,
  HostProvider,
  HostProviderConfig,
} from './host-provider-types';

const hostProviderConfigSchema = z.object({
  type: z.literal('host'),
  refreshInterval: z.coerce.number().default(60 * 1000),
});

export function createHostProvider(
  config: HostProviderConfig,
): HostProvider {
  const mergedConfig = hostProviderConfigSchema.parse(config);

  return createBaseProvider(mergedConfig, async queue => {
    return onProviderEmit<HostOutput>(mergedConfig, ({ result }) => {
      if ('error' in result) {
        queue.error(result.error);
      } else {
        queue.output(result.output);
      }
    });
  });
}
