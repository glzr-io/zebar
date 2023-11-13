import { createEffect } from 'solid-js';
import { createStore } from 'solid-js/store';

import { onProviderEmit, listenProvider, unlistenProvider } from '~/desktop';
import { HostProviderOptions, HostProviderOptionsSchema } from '~/user-config';
import { memoize, simpleHash } from '~/utils';

const DEFAULT = HostProviderOptionsSchema.parse({});

export interface HostVariables {
  isLoading: boolean;
  hostname: string | null;
  osName: string | null;
  osVersion: string | null;
  friendlyOsVersion: string | null;
  bootTime: number;
  uptime: number;
}

export const createHostProvider = memoize(
  (options: HostProviderOptions = DEFAULT) => {
    const [hostVariables, setHostVariables] = createStore<HostVariables>({
      isLoading: true,
      hostname: null,
      osName: null,
      osVersion: null,
      friendlyOsVersion: null,
      bootTime: 0,
      uptime: 0,
    });

    createEffect(async () => {
      const optionsHash = simpleHash(options);

      onProviderEmit<HostVariables>(optionsHash, payload =>
        setHostVariables({ ...payload, isLoading: false }),
      );

      await listenProvider({
        optionsHash,
        options,
        trackedAccess: [],
      });

      return () => unlistenProvider(optionsHash);
    });

    return {
      get isLoading() {
        return hostVariables.isLoading;
      },
      get hostname() {
        return hostVariables.hostname;
      },
      get osName() {
        return hostVariables.osName;
      },
      get osVersion() {
        return hostVariables.osVersion;
      },
      get friendlyOsVersion() {
        return hostVariables.friendlyOsVersion;
      },
      get bootTime() {
        return hostVariables.bootTime;
      },
      get uptime() {
        return hostVariables.uptime;
      },
    };
  },
);
