import {
  SystemTrayProviderOptions,
  SystemTrayProviderOptionsSchema,
} from '~/shared/user-config';
import { memoize } from '~/shared/utils';

const DEFAULT = SystemTrayProviderOptionsSchema.parse({});

export const useSystemTrayProvider = memoize(
  (options: SystemTrayProviderOptions = DEFAULT) => {
    return {
      variables: {
        xx: '',
      },
      commands: {},
    };
  },
);
