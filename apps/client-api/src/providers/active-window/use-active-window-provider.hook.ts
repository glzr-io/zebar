import {
  ActiveWindowProviderOptions,
  ActiveWindowProviderOptionsSchema,
} from '../../user-config';
import { memoize } from '../../utils';

const DEFAULT = ActiveWindowProviderOptionsSchema.parse({});

export const useActiveWindowProvider = memoize(
  (options: ActiveWindowProviderOptions = DEFAULT) => {
    return {
      variables: {
        title: '',
      },
      commands: {},
    };
  },
);
