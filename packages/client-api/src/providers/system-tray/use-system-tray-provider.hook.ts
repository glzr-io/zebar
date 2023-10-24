import {
  SystemTrayProviderOptions,
  SystemTrayProviderOptionsSchema,
} from '../../user-config';
import { memoize } from '../../utils';

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
