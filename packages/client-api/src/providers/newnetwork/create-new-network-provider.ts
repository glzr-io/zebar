import type { Owner } from 'solid-js';

import type { NewNetworkProviderConfig } from '~/user-config';
import { createProviderListener } from '../create-provider-listener';

export interface NewNetworkVariables {
  name: string;
  macAddress: string;
  strength: string
}

export async function createNewNetworkProvider(
  config: NewNetworkProviderConfig,
  owner: Owner,
) {
  const newNetworkVariables = await createProviderListener<
    NewNetworkProviderConfig,
    NewNetworkVariables
  >(config, owner);

  return {
    get macAddress() {
      return newNetworkVariables().macAddress;
    },
    get name() {
      return newNetworkVariables().name;
    },
    get strength() {
      return newNetworkVariables().strength;
    },
  };
}
