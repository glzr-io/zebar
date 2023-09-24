import { ActiveWindowProviderConfig } from '~/shared/user-config';
import { memoize } from '~/shared/utils';

export const useActiveWindowProvider = memoize(
  (config: ActiveWindowProviderConfig) => {
    return {
      variables: {
        title: '',
      },
      commands: {},
    };
  },
);
