import { createEffect } from 'solid-js';
import { createStore } from 'solid-js/store';

import { listenProvider, onProviderEmit, unlistenProvider } from '~/desktop';
import { BatteryProviderConfig } from '~/user-config';
import { memoize, simpleHash } from '~/utils';

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
    const [batteryVariables, setBatteryVariables] =
      createStore<BatteryVariables>({
        isLoading: true,
        chargePercent: 0,
        cycleCount: 0,
        healthPercent: 0,
        powerConsumption: 0,
        state: 'discharging',
        timeTillEmpty: null,
        timeTillFull: null,
        voltage: null,
      });

    createEffect(async () => {
      const configHash = simpleHash(config);

      onProviderEmit<BatteryVariables>(configHash, payload =>
        setBatteryVariables({ ...payload, isLoading: false }),
      );

      await listenProvider({
        configHash,
        config,
        trackedAccess: [],
      });

      return () => unlistenProvider(configHash);
    });

    return {
      get isLoading() {
        return batteryVariables.isLoading;
      },
      get chargePercent() {
        return batteryVariables.chargePercent;
      },
      get cycleCount() {
        return batteryVariables.cycleCount;
      },
      get healthPercent() {
        return batteryVariables.healthPercent;
      },
      get powerConsumption() {
        return batteryVariables.powerConsumption;
      },
      get state() {
        return batteryVariables.state;
      },
      get timeTillEmpty() {
        return batteryVariables.timeTillEmpty;
      },
      get timeTillFull() {
        return batteryVariables.timeTillFull;
      },
      get voltage() {
        return batteryVariables.voltage;
      },
    };
  },
);
