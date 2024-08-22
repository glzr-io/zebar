import {
  type Accessor,
  createEffect,
  createSignal,
  onCleanup,
  type Owner,
  runWithOwner,
} from 'solid-js';

import {
  onProviderEmit,
  listenProvider,
  unlistenProvider,
} from '~/desktop';
import { simpleHash } from '~/utils';
import type { ProviderConfig } from './provider-config.model';

/**
 * Utility for listening to a provider of a given config type.
 */
export function createProviderListener<
  TConfig extends ProviderConfig,
  TVars,
>(config: TConfig, owner: Owner): Promise<Accessor<TVars>> {
  return new Promise(async resolve => {
    const [payload, setPayload] = createSignal<TVars>();

    const configHash = simpleHash(config);
    const unlisten = await onProviderEmit<TVars>(configHash, setPayload);

    await listenProvider({
      configHash,
      config,
      trackedAccess: [],
    });

    runWithOwner(owner, () => {
      onCleanup(() => {
        unlisten();
        unlistenProvider(configHash);
      });

      createEffect(() => {
        if (payload()) {
          resolve(payload as Accessor<TVars>);
        }
      });
    });
  });
}
