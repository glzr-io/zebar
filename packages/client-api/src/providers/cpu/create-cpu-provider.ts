import { z } from 'zod';

import { createProviderListener } from '../create-provider-listener';

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

export interface CpuProvider {
  frequency: number;
  usage: number;
  logicalCoreCount: number;
  physicalCoreCount: number;
  vendor: string;
  onChange: (callback: (provider: CpuProvider) => void) => void;
}

export async function createCpuProvider(
  config: CpuProviderConfig,
): Promise<CpuProvider> {
  const mergedConfig = cpuProviderConfigSchema.parse(config);

  const { firstValue, onChange } =
    await createProviderListener<CpuProvider>(mergedConfig);

  const cpuProvider = { ...firstValue, onChange };
  onChange(incoming => Object.assign(cpuProvider, incoming));

  return cpuProvider;
}
