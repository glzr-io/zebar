import { UnlistenFn } from 'glazewm';
import { createResource, createEffect, Resource } from 'solid-js';

import { onProviderEmit, listenProvider, unlistenProvider } from '~/desktop';
import { ProviderConfig } from '~/user-config';
import { simpleHash } from '~/utils';

/**
 * Utility for creating a {@link Resource} that listens to a provider of a
 * given config type.
 */
export function createProviderListener<
  TConfig extends ProviderConfig,
  TVars extends { isLoading: boolean },
>(config: TConfig) {
  const configHash = simpleHash(config);
  const unlistenFns: UnlistenFn[] = [];

  const resource = createResource<TVars>(() => {
    return new Promise(async resolve => {
      const unlisten = await onProviderEmit<TVars>(configHash, payload =>
        resolve({ ...payload, isLoading: false }),
      );

      unlistenFns.push(unlisten);

      await listenProvider({
        configHash,
        config,
        trackedAccess: [],
      });
    });
  });

  const [_, { mutate }] = resource;

  createEffect(async () => {
    const unlisten = await onProviderEmit<TVars>(configHash, payload =>
      mutate<TVars>({ ...payload, isLoading: false } as Exclude<
        TVars,
        Function
      >),
    );

    unlistenFns.push(unlisten);

    return () => {
      unlistenProvider(configHash);
      unlistenFns.forEach(unlisten => unlisten());
    };
  });

  return resource;
}
