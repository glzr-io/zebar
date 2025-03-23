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
import { WidgetPack } from 'zebar';

import { useApiClient } from '../api-client';

type MarketplacePacksContextState = {
  allPacks: Resource<MarketplaceWidgetPack[]>;
  previewPack: Accessor<MarketplaceWidgetPack | null>;
  install: (pack: MarketplaceWidgetPack) => Promise<void>;
  startPreview: (
    pack: MarketplaceWidgetPack,
    widgetName?: string,
  ) => Promise<void>;
  stopPreview: () => Promise<void>;
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

  async function install(pack: MarketplaceWidgetPack) {
    await invoke<void>('install_widget_pack', {
      packId: pack.publishedId,
      version: pack.latestVersion,
      tarballUrl: pack.tarballUrl,
      isPreview: false,
    });
  }

  async function startPreview(
    pack: MarketplaceWidgetPack,
    widgetName?: string,
  ) {
    const installedPack = await invoke<WidgetPack>('install_widget_pack', {
      packId: pack.publishedId,
      version: pack.latestVersion,
      tarballUrl: pack.tarballUrl,
      isPreview: true,
    });

    await invoke<void>('start_widget_preset', {
      packId: pack.publishedId,
      widgetName: widgetName ?? installedPack.widgets[0].name,
      presetName: installedPack.widgets[0].presets[0].name,
      isPreview: true,
    });

    setPreviewPack(pack);
  }

  async function stopPreview() {
    await invoke<void>('stop_all_preview_widgets');
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
