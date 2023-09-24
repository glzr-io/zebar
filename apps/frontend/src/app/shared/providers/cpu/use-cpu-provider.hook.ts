import { CpuProviderConfig } from '~/shared/user-config';
import { memoize } from '../../utils';

export const useCpuProvider = memoize((config: CpuProviderConfig) => {
  return {
    variables: {
      usage: 0,
      temp: 0,
      frequency: 0,
    },
    commands: {},
  };
});
