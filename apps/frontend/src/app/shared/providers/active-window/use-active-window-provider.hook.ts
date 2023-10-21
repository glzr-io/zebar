import {
  ActiveWindowProviderOptions,
  ActiveWindowProviderOptionsSchema,
} from '~/shared/user-config';
import { memoize } from '~/shared/utils';

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
