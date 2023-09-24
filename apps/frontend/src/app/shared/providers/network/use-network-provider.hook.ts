import { NetworkProviderConfig } from '~/shared/user-config';
import { memoize } from '../../utils';

export const useNetworkProvider = memoize((config: NetworkProviderConfig) => {
  return {
    xx: '',
  };
});
