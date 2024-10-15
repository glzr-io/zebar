import { z } from 'zod';

import { createBaseProvider } from '../create-base-provider';
import { onProviderEmit } from '~/desktop';
import type {
  IpOutput,
  IpProvider,
  IpProviderConfig,
} from './ip-provider-types';

const ipProviderConfigSchema = z.object({
  type: z.literal('ip'),
  refreshInterval: z.coerce.number().default(60 * 60 * 1000),
});

export function createIpProvider(config: IpProviderConfig): IpProvider {
  const mergedConfig = ipProviderConfigSchema.parse(config);

  return createBaseProvider(mergedConfig, async queue => {
    return onProviderEmit<IpOutput>(mergedConfig, ({ result }) => {
      if ('error' in result) {
        queue.error(result.error);
      } else {
        queue.output(result.output);
      }
    });
  });
}
