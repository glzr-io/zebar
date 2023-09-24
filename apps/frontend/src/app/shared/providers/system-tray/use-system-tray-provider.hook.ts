import { SystemTrayProviderConfig } from '~/shared/user-config';
import { memoize } from '../../utils';

export const useSystemTrayProvider = memoize(
  (config: SystemTrayProviderConfig) => {
    return {};
  },
);
