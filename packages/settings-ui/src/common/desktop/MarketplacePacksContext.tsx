import type { RouterOutputs } from '@glzr/data-access';
import { invoke } from '@tauri-apps/api/core';
import {
  createContext,
  createResource,
  type JSX,
  Resource,
  useContext,
} from 'solid-js';

import { useApiClient } from '../api-client';

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

  // Fetch marketplace packs from the backend.
  const [allPacks] = createResource(
    async () => {
      return apiClient.widgetPack.getAll.query();
    },
    { initialValue: [] },
  );

  async function install(pack: MarketplaceWidgetPack) {
    await invoke<void>('install_widget_pack', {
      packId: pack.publishedId,
      version: pack.latestVersion,
      tarballUrl: pack.tarballUrl,
      isPreview: false,
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
