import { NetworkProviderConfig } from '~/user-config';
import { memoize } from '~/utils';
import { createProviderListener } from '../create-provider-listener';

export interface NetworkVariables {
  isLoading: boolean;
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

export const createNetworkProvider = memoize(
  (config: NetworkProviderConfig) => {
    const [networkVariables] = createProviderListener<
      NetworkProviderConfig,
      NetworkVariables
    >(config);

    return {
      get isLoading() {
        return networkVariables()?.isLoading ?? true;
      },
      get interfaces() {
        return networkVariables()?.interfaces;
      },
    };
  },
);
