import { z } from 'zod';

import { createBaseProvider } from '../create-base-provider';
import { onProviderEmit } from '~/desktop';
import type {
  NetworkOutput,
  NetworkProvider,
  NetworkProviderConfig,
} from './network-provider-types';

const networkProviderConfigSchema = z.object({
  type: z.literal('network'),
  refreshInterval: z.coerce.number().default(5 * 1000),
});

export function createNetworkProvider(
  config: NetworkProviderConfig,
): NetworkProvider {
  const mergedConfig = networkProviderConfigSchema.parse(config);

  return createBaseProvider(mergedConfig, async queue => {
    return onProviderEmit<NetworkOutput>(mergedConfig, ({ result }) => {
      if ('error' in result) {
        queue.error(result.error);
      } else {
        queue.output(result.output);
      }
    });
  });
}
