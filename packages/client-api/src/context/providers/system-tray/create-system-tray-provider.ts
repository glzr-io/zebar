import {
  SystemTrayProviderOptions,
  SystemTrayProviderOptionsSchema,
} from '~/user-config';
import { memoize } from '~/utils';

const DEFAULT = SystemTrayProviderOptionsSchema.parse({});

export const createSystemTrayProvider = memoize(
  (options: SystemTrayProviderOptions = DEFAULT) => {
    return {
      xx: '',
    };
  },
);
