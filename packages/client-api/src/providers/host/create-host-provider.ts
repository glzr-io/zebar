import { z } from 'zod';

import {
  createBaseProvider,
  type Provider,
} from '../create-base-provider';
import { onProviderEmit } from '~/desktop';

export interface HostProviderConfig {
  type: 'host';

  /**
   * How often this provider refreshes in milliseconds.
   */
  refreshInterval?: number;
}

const hostProviderConfigSchema = z.object({
  type: z.literal('host'),
  refreshInterval: z.coerce.number().default(60 * 1000),
});

export type HostProvider = Provider<HostProviderConfig, HostOutput>;

export interface HostOutput {
  hostname: string | null;
  osName: string | null;
  osVersion: string | null;
  friendlyOsVersion: string | null;
  bootTime: number;
  uptime: number;
}

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
