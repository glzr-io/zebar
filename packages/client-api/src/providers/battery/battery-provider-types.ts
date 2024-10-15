import type { Provider } from '../create-base-provider';

export interface BatteryProviderConfig {
  type: 'battery';

  /**
   * How often this provider refreshes in milliseconds.
   */
  refreshInterval?: number;
}

export type BatteryProvider = Provider<
  BatteryProviderConfig,
  BatteryOutput
>;

export interface BatteryOutput {
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
