import type { Owner } from 'solid-js';
import { z } from 'zod';

import { createProviderListener } from '../create-provider-listener';

export interface IpProviderConfig {
  type: 'ip';

  /**
   * How often this provider refreshes in milliseconds.
   */
  refreshInterval?: number;
}

const IpProviderConfigSchema = z.object({
  type: z.literal('ip'),
  refreshInterval: z.coerce.number().default(60 * 60 * 1000),
});

export interface IpProvider {
  address: string;
  approxCity: string;
  approxCountry: string;
  approxLatitude: number;
  approxLongitude: number;
}

export async function createIpProvider(
  config: IpProviderConfig,
  owner: Owner,
) {
  const mergedConfig = IpProviderConfigSchema.parse(config);

  const ipVariables = await createProviderListener<
    IpProviderConfig,
    IpProvider
  >(mergedConfig, owner);

  return {
    get address() {
      return ipVariables().address;
    },
    get approxCity() {
      return ipVariables().approxCity;
    },
    get approxCountry() {
      return ipVariables().approxCountry;
    },
    get approxLatitude() {
      return ipVariables().approxLatitude;
    },
    get approxLongitude() {
      return ipVariables().approxLongitude;
    },
  };
}
