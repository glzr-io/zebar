import {
  BatteryProviderOptions,
  BatteryProviderOptionsSchema,
} from '~/shared/user-config';
import { memoize } from '~/shared/utils';

const DEFAULT = BatteryProviderOptionsSchema.parse({});

export const useBatteryProvider = memoize(
  (options: BatteryProviderOptions = DEFAULT) => {
    return {
      variables: {
        percent: '',
        is_charging: true,
        has_battery: true,
      },
      commands: {},
    };
  },
);
