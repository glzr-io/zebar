import type { RouterOutputs } from '@glzr/data-access';
import { invoke } from '@tauri-apps/api/core';
import { createContext, type JSX, Resource, useContext } from 'solid-js';
import type { WidgetPack } from 'zebar';

import { useApiClient } from '../api-client';
import { useUserPacks } from './UserPacksContext';
import { createSafeResource } from '../create-safe-resource';

type MarketplacePacksContextState = {
  allPacks: Resource<MarketplaceWidgetPack[]>;
  install: (pack: MarketplaceWidgetPack) => Promise<void>;
};

export type MarketplaceWidgetPack =
  RouterOutputs['widgetPack']['getAll'][number];

const MarketplacePacksContext =
  createContext<MarketplacePacksContextState>();

export function MarketplacePacksProvider(props: {
  children: JSX.Element;
}) {
  const apiClient = useApiClient();
  const userPacks = useUserPacks();

  // Fetch marketplace packs from the backend.
  const [allPacks] = createSafeResource(async () =>
    apiClient.widgetPack.getAll.query(),
  );

  async function install(pack: MarketplaceWidgetPack) {
    const installedPack = await invoke<WidgetPack>('install_widget_pack', {
      packId: pack.publishedId,
      version: pack.latestVersion,
      tarballUrl: pack.tarballUrl,
      isPreview: false,
    });

    userPacks.mutatePacks(packs => {
      const existingPackIndex = packs?.findIndex(
        pack => pack.id === installedPack.id,
      );

      // Update in place if the pack already exists. Otherwise, append the
      // new pack.
      if (existingPackIndex !== undefined && existingPackIndex >= 0) {
        const updatedPacks = [...(packs ?? [])];
        updatedPacks[existingPackIndex] = installedPack;
        return updatedPacks;
      } else {
        return [...(packs ?? []), installedPack];
      }
    });
  }

  const store: MarketplacePacksContextState = {
    allPacks,
    install,
  };

  return (
    <MarketplacePacksContext.Provider value={store}>
      {props.children}
    </MarketplacePacksContext.Provider>
  );
}

export function useMarketplacePacks() {
  const context = useContext(MarketplacePacksContext);

  if (!context) {
    throw new Error(
      '`useMarketplacePacks` must be used within a `MarketplacePacksProvider`.',
    );
  }

  return context;
}
