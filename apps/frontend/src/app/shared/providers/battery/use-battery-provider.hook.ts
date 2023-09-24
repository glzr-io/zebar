import { BatteryProviderConfig } from '~/shared/user-config';
import { memoize } from '../../utils';

export const useBatteryProvider = memoize((config: BatteryProviderConfig) => {
  return {
    variables: {
      percent: '',
      is_charging: true,
      has_battery: true,
    },
    commands: {},
  };
});
