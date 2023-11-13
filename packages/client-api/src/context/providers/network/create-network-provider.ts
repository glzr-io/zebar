import { createEffect } from 'solid-js';
import { createStore } from 'solid-js/store';

import { onProviderEmit, listenProvider, unlistenProvider } from '~/desktop';
import { NetworkProviderConfig } from '~/user-config';
import { memoize, simpleHash } from '~/utils';

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
    const [networkVariables, setNetworkVariables] =
      createStore<NetworkVariables>({
        isLoading: true,
        interfaces: [],
      });

    createEffect(async () => {
      const configHash = simpleHash(config);

      onProviderEmit<NetworkVariables>(configHash, payload =>
        setNetworkVariables({ ...payload, isLoading: false }),
      );

      await listenProvider({
        configHash: configHash,
        config: config,
        trackedAccess: [],
      });

      return () => unlistenProvider(configHash);
    });

    return {
      get isLoading() {
        return networkVariables.isLoading;
      },
      get interfaces() {
        return networkVariables.interfaces;
      },
    };
  },
);
