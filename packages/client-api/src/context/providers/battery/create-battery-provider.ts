import { BatteryProviderConfig } from '~/user-config';
import { memoize } from '~/utils';
import { createProviderListener } from '../create-provider-listener';

export interface BatteryVariables {
  isLoading: boolean;
  chargePercent: number;
  cycleCount: number;
  healthPercent: number;
  powerConsumption: number;
  state: 'discharging' | 'charging';
  timeTillEmpty: number | null;
  timeTillFull: number | null;
  voltage: number | null;
}

export const createBatteryProvider = memoize(
  (config: BatteryProviderConfig) => {
    const [batteryVariables] = createProviderListener<
      BatteryProviderConfig,
      BatteryVariables
    >(config);

    return {
      get isLoading() {
        return batteryVariables()?.isLoading ?? true;
      },
      get chargePercent() {
        return batteryVariables()?.chargePercent;
      },
      get cycleCount() {
        return batteryVariables()?.cycleCount;
      },
      get healthPercent() {
        return batteryVariables()?.healthPercent;
      },
      get powerConsumption() {
        return batteryVariables()?.powerConsumption;
      },
      get state() {
        return batteryVariables()?.state;
      },
      get timeTillEmpty() {
        return batteryVariables()?.timeTillEmpty;
      },
      get timeTillFull() {
        return batteryVariables()?.timeTillFull;
      },
      get voltage() {
        return batteryVariables()?.voltage;
      },
    };
  },
);
