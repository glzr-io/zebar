import {
  NetworkProviderOptions,
  NetworkProviderOptionsSchema,
} from '~/shared/user-config';
import { memoize } from '~/shared/utils';

const DEFAULT = NetworkProviderOptionsSchema.parse({});

export const useNetworkProvider = memoize(
  (options: NetworkProviderOptions = DEFAULT) => {
    return {
      variables: {
        xx: '',
      },
      commands: {},
    };
  },
);
