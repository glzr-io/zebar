import {
  BatteryProviderOptions,
  BatteryProviderOptionsSchema,
} from '~/user-config';
import { memoize } from '~/utils';

const DEFAULT = BatteryProviderOptionsSchema.parse({});

export const createBatteryProvider = memoize(
  (options: BatteryProviderOptions = DEFAULT) => {
    return {
      percent: '',
      is_charging: true,
      has_battery: true,
    };
  },
);
