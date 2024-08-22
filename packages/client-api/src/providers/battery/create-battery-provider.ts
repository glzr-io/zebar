import type { Owner } from 'solid-js';

import { createProviderListener } from '../create-provider-listener';
import type { ProviderType } from '../provider-type.model';

export interface BatteryProviderConfig {
  type: ProviderType.BATTERY;

  /**
   * How often this provider refreshes in milliseconds.
   */
  refreshInterval?: number;
}

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
  const batteryVariables = await createProviderListener<
    BatteryProviderConfig,
    BatteryProvider
  >(config, owner);

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
