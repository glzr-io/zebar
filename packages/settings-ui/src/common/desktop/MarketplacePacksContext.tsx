import type { RouterOutputs } from '@glzr/data-access';
import { invoke } from '@tauri-apps/api/core';
import {
  Accessor,
  createContext,
  createSignal,
  type JSX,
  Resource,
  useContext,
} from 'solid-js';
import { createResource } from 'solid-js';

import { useApiClient } from '../api-client';

type MarketplacePacksContextState = {
  allPacks: Resource<MarketplaceWidgetPack[]>;
  previewPack: Accessor<MarketplaceWidgetPack | null>;
  install: (pack: MarketplaceWidgetPack) => void;
  startPreview: (pack: MarketplaceWidgetPack) => void;
  stopPreview: () => void;
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

  const [previewPack, setPreviewPack] =
    createSignal<MarketplaceWidgetPack | null>(null);

  function install(pack: MarketplaceWidgetPack) {
    invoke<void>('install_widget_pack', {
      packId: pack.id,
      version: pack.latestVersion,
      tarballUrl: pack.tarballUrl,
    });
  }

  function startPreview(pack: MarketplaceWidgetPack) {
    invoke<void>('preview_widget_pack', {
      packId: pack.id,
      version: pack.latestVersion,
      tarballUrl: pack.tarballUrl,
    });

    setPreviewPack(pack);
  }

  function stopPreview() {
    invoke<void>('stop_preview_widget_pack', {
      packId: previewPack()?.id,
    });

    setPreviewPack(null);
  }

  const store: MarketplacePacksContextState = {
    allPacks,
    previewPack,
    startPreview,
    stopPreview,
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
