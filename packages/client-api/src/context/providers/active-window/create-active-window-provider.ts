import {
  ActiveWindowProviderOptions,
  ActiveWindowProviderOptionsSchema,
} from '~/user-config';
import { memoize } from '~/utils';

const DEFAULT = ActiveWindowProviderOptionsSchema.parse({});

export const createActiveWindowProvider = memoize(
  (options: ActiveWindowProviderOptions = DEFAULT) => {
    return {
      title: '',
    };
  },
);
