import {
  NetworkProviderOptions,
  NetworkProviderOptionsSchema,
} from '~/user-config';
import { memoize } from '~/utils';

const DEFAULT = NetworkProviderOptionsSchema.parse({});

export const createNetworkProvider = memoize(
  (options: NetworkProviderOptions = DEFAULT) => {
    return {
      xx: '',
    };
  },
);
