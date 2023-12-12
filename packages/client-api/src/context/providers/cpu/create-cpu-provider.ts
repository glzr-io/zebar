import { UnlistenFn } from '@tauri-apps/api/event';
import { createEffect, createResource } from 'solid-js';

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
  const configHash = simpleHash(config);
  const unlistenFns: UnlistenFn[] = [];

  const [cpuVariables, { mutate }] = createResource<CpuVariables>(() => {
    return new Promise(async resolve => {
      const unlisten = await onProviderEmit<CpuVariables>(configHash, payload =>
        resolve({ ...payload, isLoading: false }),
      );

      unlistenFns.push(unlisten);

      await listenProvider({
        configHash: configHash,
        config: config,
        trackedAccess: [],
      });
    });
  });

  createEffect(async () => {
    const unlisten = await onProviderEmit<CpuVariables>(configHash, payload =>
      mutate({ ...payload, isLoading: false }),
    );

    unlistenFns.push(unlisten);

    return () => {
      unlistenProvider(configHash);
      unlistenFns.forEach(unlisten => unlisten());
    };
  });

  return {
    get isLoading() {
      return cpuVariables()?.isLoading ?? true;
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
