import {
  MemoryProviderOptions,
  MemoryProviderOptionsSchema,
} from '~/shared/user-config';
import { memoize } from '~/shared/utils';

const DEFAULT = MemoryProviderOptionsSchema.parse({});

export const useMemoryProvider = memoize(
  (options: MemoryProviderOptions = DEFAULT) => {
    return {
      variables: {
        usage: 0,
      },
      commands: {},
    };
  },
);
