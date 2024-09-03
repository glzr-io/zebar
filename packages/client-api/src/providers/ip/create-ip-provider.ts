import { z } from 'zod';

import {
  createBaseProvider,
  type Provider,
} from '../create-base-provider';
import { onProviderEmit } from '~/desktop';

export interface IpProviderConfig {
  type: 'ip';

  /**
   * How often this provider refreshes in milliseconds.
   */
  refreshInterval?: number;
}

const ipProviderConfigSchema = z.object({
  type: z.literal('ip'),
  refreshInterval: z.coerce.number().default(60 * 60 * 1000),
});

export type IpProvider = Provider<IpProviderConfig, IpOutput>;

export interface IpOutput {
  address: string;
  approxCity: string;
  approxCountry: string;
  approxLatitude: number;
  approxLongitude: number;
}

export async function createIpProvider(
  config: IpProviderConfig,
): Promise<IpProvider> {
  const mergedConfig = ipProviderConfigSchema.parse(config);

  return createBaseProvider(mergedConfig, async queue => {
    return onProviderEmit<IpOutput>(mergedConfig, ({ variables }) => {
      if ('error' in variables) {
        queue.error(variables.error);
      } else {
        queue.value(variables.data);
      }
    });
  });
}
