import { createEffect } from 'solid-js';
import { createStore } from 'solid-js/store';

import { listenProvider, onProviderEmit, unlistenProvider } from '~/desktop';
import {
  BatteryProviderOptions,
  BatteryProviderOptionsSchema,
} from '~/user-config';
import { memoize } from '~/utils';

const DEFAULT = BatteryProviderOptionsSchema.parse({});

export const createBatteryProvider = memoize(
  (options: BatteryProviderOptions = DEFAULT) => {
    const [batteryData, setBatteryData] = createStore({
      percent: '',
      is_charging: true,
      has_battery: true,
    });

    createEffect(async () => {
      const configHash = simpleHash(options);
      await listenProvider(configHash, options, []);

      onProviderEmit<typeof batteryData>(configHash, payload =>
        setBatteryData(payload),
      );

      return () => unlistenProvider(configHash);
    });

    return {
      get percent() {
        return batteryData.percent;
      },
      get is_charging() {
        return batteryData.is_charging;
      },
      get has_battery() {
        return batteryData.has_battery;
      },
    };
  },
);
