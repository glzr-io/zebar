import { SystemTrayProviderConfig } from '~/user-config';
import { memoize } from '~/utils';

export const createSystemTrayProvider = memoize(
  (config: SystemTrayProviderConfig) => {
    return {
      xx: '',
    };
  },
);
