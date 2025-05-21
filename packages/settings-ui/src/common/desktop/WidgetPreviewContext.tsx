import { invoke } from '@tauri-apps/api/core';
import {
  Accessor,
  createContext,
  createSignal,
  type JSX,
  useContext,
} from 'solid-js';
import { WidgetPack } from 'zebar';

import { MarketplaceWidgetPack } from './MarketplacePacksContext';

type WidgetPreviewContextState = {
  previewPack: Accessor<WidgetPack | null>;
  previewWidgetName: Accessor<string | null>;
  previewPresetName: Accessor<string | null>;
  startPreview: (
    pack: MarketplaceWidgetPack,
    widgetName?: string,
    presetName?: string,
  ) => Promise<void>;
  changePreview: (widgetName: string, presetName: string) => Promise<void>;
  stopPreview: () => Promise<void>;
};

const WidgetPreviewContext = createContext<WidgetPreviewContextState>();

export function WidgetPreviewProvider(props: { children: JSX.Element }) {
  const [previewPack, setPreviewPack] = createSignal<WidgetPack | null>(
    null,
  );

  const [previewWidgetName, setPreviewWidgetName] = createSignal<
    string | null
  >(null);

  const [previewPresetName, setPreviewPresetName] = createSignal<
    string | null
  >(null);

  async function startPreview(
    pack: MarketplaceWidgetPack,
    widgetName?: string,
    presetName?: string,
  ) {
    const installedPack = await invoke<WidgetPack>('install_widget_pack', {
      packId: pack.publishedId,
      version: pack.latestVersion,
      tarballUrl: pack.tarballUrl,
      isPreview: true,
    });

    setPreviewWidgetName(widgetName ?? installedPack.widgets[0].name);
    setPreviewPresetName(
      presetName ?? installedPack.widgets[0].presets[0].name,
    );

    await invoke<void>('start_preview_widget', {
      packConfig: installedPack,
      widgetName: previewWidgetName(),
      presetName: previewPresetName(),
    });

    setPreviewPack(installedPack);
  }

  async function changePreview(widgetName: string, presetName: string) {
    setPreviewWidgetName(widgetName);
    setPreviewPresetName(presetName);

    await invoke<void>('stop_all_preview_widgets');

    await invoke<void>('start_preview_widget', {
      packConfig: previewPack(),
      widgetName: previewWidgetName(),
      presetName: previewPresetName(),
    });
  }

  async function stopPreview() {
    await invoke<void>('stop_all_preview_widgets');
    setPreviewPack(null);
  }

  const store: WidgetPreviewContextState = {
    previewPack,
    previewWidgetName,
    previewPresetName,
    startPreview,
    changePreview,
    stopPreview,
  };

  return (
    <WidgetPreviewContext.Provider value={store}>
      {props.children}
    </WidgetPreviewContext.Provider>
  );
}

export function useWidgetPreview() {
  const context = useContext(WidgetPreviewContext);

  if (!context) {
    throw new Error(
      '`useWidgetPreview` must be used within a `WidgetPreviewProvider`.',
    );
  }

  return context;
}
