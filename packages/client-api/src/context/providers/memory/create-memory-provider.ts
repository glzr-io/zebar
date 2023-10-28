import {
  MemoryProviderOptions,
  MemoryProviderOptionsSchema,
} from '~/user-config';
import { memoize } from '~/utils';

const DEFAULT = MemoryProviderOptionsSchema.parse({});

export const createMemoryProvider = memoize(
  (options: MemoryProviderOptions = DEFAULT) => {
    return {
      usage: 0,
    };
  },
);
