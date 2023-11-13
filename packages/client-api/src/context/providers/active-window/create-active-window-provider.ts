import { ActiveWindowProviderConfig } from '~/user-config';
import { memoize } from '~/utils';

// TODO: Implement provider.
export const createActiveWindowProvider = memoize(
  (config: ActiveWindowProviderConfig) => {
    return {
      title: '',
    };
  },
);
