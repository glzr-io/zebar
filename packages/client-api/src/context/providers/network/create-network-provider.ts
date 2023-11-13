import { createEffect } from 'solid-js';
import { createStore } from 'solid-js/store';

import { onProviderEmit, listenProvider, unlistenProvider } from '~/desktop';
import {
  NetworkProviderOptions,
  NetworkProviderOptionsSchema,
} from '~/user-config';
import { memoize, simpleHash } from '~/utils';

const DEFAULT = NetworkProviderOptionsSchema.parse({});

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
  (options: NetworkProviderOptions = DEFAULT) => {
    const [networkVariables, setNetworkVariables] =
      createStore<NetworkVariables>({
        isLoading: true,
        interfaces: [],
      });

    createEffect(async () => {
      const optionsHash = simpleHash(options);

      onProviderEmit<NetworkVariables>(optionsHash, payload =>
        setNetworkVariables({ ...payload, isLoading: false }),
      );

      await listenProvider({
        optionsHash,
        options,
        trackedAccess: [],
      });

      return () => unlistenProvider(optionsHash);
    });

    return {
      get interfaces() {
        return networkVariables.interfaces;
      },
    };
  },
);
