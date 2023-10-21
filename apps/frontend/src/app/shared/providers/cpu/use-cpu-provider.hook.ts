import {
  CpuProviderOptions,
  CpuProviderOptionsSchema,
} from '~/shared/user-config';
import { memoize } from '~/shared/utils';

const DEFAULT = CpuProviderOptionsSchema.parse({});

export const useCpuProvider = memoize(
  (options: CpuProviderOptions = DEFAULT) => {
    return {
      variables: {
        usage: 0,
        temp: 0,
        frequency: 0,
      },
      commands: {},
    };
  },
);
