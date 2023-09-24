import { MemoryProviderConfig } from '~/shared/user-config';
import { memoize } from '../../utils';

export const useMemoryProvider = memoize((config: MemoryProviderConfig) => {
  return {
    usage: 0,
  };
});
