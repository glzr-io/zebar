import { z } from 'zod';

import { createProviderListener } from '../create-provider-listener';

export interface BatteryProviderConfig {
  type: 'battery';

  /**
   * How often this provider refreshes in milliseconds.
   */
  refreshInterval?: number;
}

const BatteryProviderConfigSchema = z.object({
  type: z.literal('battery'),
  refreshInterval: z.coerce.number().default(60 * 60 * 1000),
});

export interface BatteryProvider {
  chargePercent: number;
  cycleCount: number;
  healthPercent: number;
  powerConsumption: number;
  state: 'discharging' | 'charging' | 'full' | 'empty' | 'unknown';
  isCharging: boolean;
  timeTillEmpty: number | null;
  timeTillFull: number | null;
  voltage: number | null;
  onChange: (provider: BatteryProvider) => void;
}

export async function createBatteryProvider(
  config: BatteryProviderConfig,
) {
  const mergedConfig = BatteryProviderConfigSchema.parse(config);

  const { firstValue, onChange } =
    await createProviderListener<BatteryProvider>(mergedConfig);

  const batteryVariables = firstValue;
  onChange(incoming => Object.assign(batteryVariables, incoming));

  return batteryVariables;
}
