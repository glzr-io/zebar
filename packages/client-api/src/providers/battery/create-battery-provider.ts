import type { Owner } from 'solid-js';
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
}

export async function createBatteryProvider(
  config: BatteryProviderConfig,
  owner: Owner,
) {
  const mergedConfig = BatteryProviderConfigSchema.parse(config);

  const batteryVariables = await createProviderListener<
    BatteryProviderConfig,
    BatteryProvider
  >(mergedmergedConfig, owner);

  return {
    get chargePercent() {
      return batteryVariables().chargePercent;
    },
    get cycleCount() {
      return batteryVariables().cycleCount;
    },
    get healthPercent() {
      return batteryVariables().healthPercent;
    },
    get powerConsumption() {
      return batteryVariables().powerConsumption;
    },
    get state() {
      return batteryVariables().state;
    },
    get isCharging() {
      return batteryVariables().isCharging;
    },
    get timeTillEmpty() {
      return batteryVariables().timeTillEmpty;
    },
    get timeTillFull() {
      return batteryVariables().timeTillFull;
    },
    get voltage() {
      return batteryVariables().voltage;
    },
  };
}
