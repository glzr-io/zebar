import {
  MemoryProviderOptions,
  MemoryProviderOptionsSchema,
} from '~/user-config';
import { memoize } from '~/utils';

const DEFAULT = MemoryProviderOptionsSchema.parse({});

export const createMemoryProvider = memoize(
  (options: MemoryProviderOptions = DEFAULT) => {
    return {
      variables: {
        usage: 0,
      },
      commands: {},
    };
  },
);
