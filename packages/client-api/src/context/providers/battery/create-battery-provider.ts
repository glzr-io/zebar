import { Owner } from 'solid-js';

import { BatteryProviderConfig } from '~/user-config';
import { createProviderListener } from '../create-provider-listener';

export interface BatteryVariables {
  chargePercent: number;
  cycleCount: number;
  healthPercent: number;
  powerConsumption: number;
  state: 'discharging' | 'charging';
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
    BatteryVariables
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
