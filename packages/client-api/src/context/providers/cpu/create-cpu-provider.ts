import { createEffect, createResource } from 'solid-js';
import { createStore } from 'solid-js/store';

import { onProviderEmit, listenProvider, unlistenProvider } from '~/desktop';
import { CpuProviderConfig } from '~/user-config';
import { memoize, simpleHash } from '~/utils';

export interface CpuVariables {
  isLoading: boolean;
  frequency: number;
  usage: number;
  logicalCoreCount: number;
  physicalCoreCount: number;
  vendor: string;
}

export const createCpuProvider = memoize((config: CpuProviderConfig) => {
  const [cpuVariables] = createResource<CpuVariables>(() => {
    return new Promise(async resolve => {
      setTimeout(async () => {
        const configHash = simpleHash(config);

        onProviderEmit<CpuVariables>(configHash, payload =>
          resolve({ ...payload, isLoading: false }),
        );

        await listenProvider({
          configHash: configHash,
          config: config,
          trackedAccess: [],
        });
      }, 7000);
    });
  });

  return {
    get isLoading() {
      return cpuVariables()?.isLoading;
    },
    get frequency() {
      return cpuVariables()?.frequency;
    },
    get usage() {
      return cpuVariables()?.usage;
    },
    get logicalCoreCount() {
      return cpuVariables()?.logicalCoreCount;
    },
    get physicalCoreCount() {
      return cpuVariables()?.physicalCoreCount;
    },
    get vendor() {
      return cpuVariables()?.vendor;
    },
  };
});
