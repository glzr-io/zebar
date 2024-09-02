import { z } from 'zod';

import { createProviderListener } from '../create-provider-listener';
import {
  createBaseProvider,
  type Provider,
} from '../create-base-provider';

export interface CpuProviderConfig {
  type: 'cpu';

  /**
   * How often this provider refreshes in milliseconds.
   */
  refreshInterval?: number;
}

const cpuProviderConfigSchema = z.object({
  type: z.literal('cpu'),
  refreshInterval: z.coerce.number().default(5 * 1000),
});

export type CpuProvider = Provider<CpuProviderConfig, CpuValues>;

export interface CpuValues {
  frequency: number;
  usage: number;
  logicalCoreCount: number;
  physicalCoreCount: number;
  vendor: string;
}

export async function createCpuProvider(
  config: CpuProviderConfig,
): Promise<CpuProvider> {
  const mergedConfig = cpuProviderConfigSchema.parse(config);

  return createBaseProvider(mergedConfig, async queue => {
    const { firstValue, onChange, unlisten } =
      await createProviderListener<CpuValues>(mergedConfig);

    queue.value(firstValue);
    onChange(val => queue.value(val));

    return async () => {
      await unlisten();
    };
  });
}
