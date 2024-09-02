import { z } from 'zod';

import { createProviderListener } from '../create-provider-listener';

export interface CpuProviderConfig {
  type: 'cpu';

  /**
   * How often this provider refreshes in milliseconds.
   */
  refreshInterval?: number;
}

const CpuProviderConfigSchema = z.object({
  type: z.literal('cpu'),
  refreshInterval: z.coerce.number().default(5 * 1000),
});

export interface CpuProvider {
  frequency: number;
  usage: number;
  logicalCoreCount: number;
  physicalCoreCount: number;
  vendor: string;
}

export async function createCpuProvider(config: CpuProviderConfig) {
  const mergedConfig = CpuProviderConfigSchema.parse(config);

  const { firstValue, onChange } =
    await createProviderListener<CpuProvider>(mergedConfig);

  const cpuVariables = { ...firstValue, onChange };
  onChange(incoming => Object.assign(cpuVariables, incoming));

  return cpuVariables;
}
