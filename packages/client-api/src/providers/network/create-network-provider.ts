import { Owner } from 'solid-js';

import { NetworkProviderConfig } from '~/user-config';
import { createProviderListener } from '../create-provider-listener';

export interface NetworkVariables {
  interfaces: NetworkInterface[];
}

export interface NetworkInterface {
  name: string;
  macAddress: string;
  transmitted: number;
  totalTransmitted: number;
  received: number;
  totalReceived: number;
}

export async function createNetworkProvider(
  config: NetworkProviderConfig,
  owner: Owner,
) {
  const networkVariables = await createProviderListener<
    NetworkProviderConfig,
    NetworkVariables
  >(config, owner);

  return {
    get interfaces() {
      return networkVariables().interfaces;
    },
  };
}
