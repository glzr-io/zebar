import type { Owner } from 'solid-js';

import type { NetworkActivityProviderConfig } from '~/user-config';
import { createProviderListener } from '../create-provider-listener';

export interface NetworkActivityVariables {
  received: number;
  transmitted: number;
}

export async function createNetworkActivityProvider(
  config: NetworkActivityProviderConfig,
  owner: Owner,
) {
  const networkActivityVariables = await createProviderListener<
    NetworkActivityProviderConfig,
    NetworkActivityVariables
  >(config, owner);

  return {
    get received() {
      return networkActivityVariables().received;
    },
    get transmitted() {
      return networkActivityVariables().transmitted;
    },
  };
}
