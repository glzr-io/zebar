import { createEffect } from 'solid-js';
import { createStore } from 'solid-js/store';

import { listenProvider, onProviderEmit, unlistenProvider } from '~/desktop';
import {
  BatteryProviderOptions,
  BatteryProviderOptionsSchema,
} from '~/user-config';
import { memoize, simpleHash } from '~/utils';

const DEFAULT = BatteryProviderOptionsSchema.parse({});

export const createBatteryProvider = memoize(
  (options: BatteryProviderOptions = DEFAULT) => {
    const [batteryData, setBatteryData] = createStore({
      charge_percent: 0,
      health_percent: 0,
      state: 0,
      time_till_full: 0,
      time_till_empty: 0,
      power_consumption: 0,
      voltage: 0,
      cycle_count: 0,
    });

    createEffect(async () => {
      const optionsHash = simpleHash(options);

      await listenProvider({
        optionsHash,
        options,
        trackedAccess: [],
      });

      onProviderEmit<typeof batteryData>(optionsHash, payload =>
        setBatteryData(payload),
      );

      return () => unlistenProvider(optionsHash);
    });

    return {
      get charge_percent() {
        return batteryData.charge_percent;
      },
      get health_percent() {
        return batteryData.health_percent;
      },
      get state() {
        return batteryData.state;
      },
      get time_till_full() {
        return batteryData.time_till_full;
      },
      get time_till_empty() {
        return batteryData.time_till_empty;
      },
      get power_consumption() {
        return batteryData.power_consumption;
      },
      get voltage() {
        return batteryData.voltage;
      },
      get cycle_count() {
        return batteryData.cycle_count;
      },
    };
  },
);
