import {
  createContext,
  type JSX,
  Resource,
  useContext,
  createEffect,
} from 'solid-js';
import { invoke } from '@tauri-apps/api/core';
import { listen, type Event } from '@tauri-apps/api/event';
import { createResource } from 'solid-js';
import type { Widget, WidgetConfig, WidgetPack } from 'zebar';

export type CreateWidgetPackArgs = {
  name: string;
  description: string;
  tags: string[];
  excludeFiles: string;
};

export type UpdateWidgetPackArgs = {
  name?: string;
  description?: string;
  tags?: string[];
  previewImages?: string[];
  excludeFiles?: string;
  widgets?: string[];
};

export type CreateWidgetArgs = {
  name: string;
  packId: string;
  template: 'react_buildless' | 'solid_typescript';
};

type UserPacksContextState = {
  downloadedPacks: Resource<WidgetPack[]>;
  localPacks: Resource<WidgetPack[]>;
  allPacks: Resource<WidgetPack[]>;
  widgetStates: Resource<Record<string, Widget>>;
  createPack: (args: CreateWidgetPackArgs) => Promise<WidgetPack>;
  createWidget: (args: CreateWidgetArgs) => Promise<WidgetConfig>;
  updatePack: (
    packId: string,
    args: UpdateWidgetPackArgs,
  ) => Promise<WidgetPack>;
  deletePack: (packId: string) => Promise<void>;
  deleteWidget: (packId: string, widgetName: string) => Promise<void>;
  updateWidgetConfig: (
    packId: string,
    widgetName: string,
    newConfig: WidgetConfig,
  ) => Promise<WidgetConfig>;
  togglePreset: (
    packId: string,
    widgetName: string,
    presetName: string,
  ) => Promise<void>;
};

const UserPacksContext = createContext<UserPacksContextState>();

export function UserPacksProvider(props: { children: JSX.Element }) {
  const [allPacks, { mutate: mutatePacks }] = createResource(async () =>
    invoke<WidgetPack[]>('widget_packs'),
  );

  const [downloadedPacks] = createResource(allPacks, packs =>
    packs?.filter(pack => pack.type === 'marketplace'),
  );

  const [localPacks] = createResource(allPacks, packs =>
    packs?.filter(pack => pack.type === 'local'),
  );

  createEffect(() => {
    console.log('localPacks', localPacks());
    console.log('downloadedPacks', downloadedPacks());
  });

  const [widgetStates, { mutate: mutateWidgetStates }] = createResource(
    async () => invoke<Record<string, Widget>>('widget_states'),
    { initialValue: {} },
  );

  // Update widget states on open.
  listen('widget-opened', (event: Event<any>) => {
    mutateWidgetStates(states => ({
      ...states,
      [event.payload.id]: event.payload,
    }));
  });

  // Update widget states on close.
  listen('widget-closed', (event: Event<any>) => {
    mutateWidgetStates(states => {
      const newStates = { ...states };
      delete newStates[event.payload];
      return newStates;
    });
  });

  async function updateWidgetConfig(
    packId: string,
    widgetName: string,
    newConfig: WidgetConfig,
  ) {
    const updatedEntry = await invoke<WidgetConfig>(
      'update_widget_config',
      {
        packId,
        widgetName,
        newConfig,
      },
    );

    mutatePacks(packs =>
      packs?.map(pack =>
        pack.id === packId && pack.type === 'local'
          ? {
              ...pack,
              widgets: pack.widgets.map(configEntry =>
                configEntry.name === widgetName
                  ? updatedEntry
                  : configEntry,
              ),
            }
          : pack,
      ),
    );

    return updatedEntry;
  }

  async function togglePreset(
    packId: string,
    widgetName: string,
    presetName: string,
  ) {
    const states = widgetStates();
    console.log(states);

    const configStates = Object.values(states).filter(
      state => state.packId === packId && state.name === widgetName,
    );

    const presetStates = configStates.filter(
      // @ts-ignore - TODO
      state => state.openOptions?.preset === presetName,
    );

    if (presetStates.length > 0) {
      await invoke<void>('stop_widget_preset', {
        packId,
        widgetName,
        presetName,
        isPreview: false,
      });
    } else {
      await invoke<void>('start_widget_preset', {
        packId,
        widgetName,
        presetName,
        isPreview: false,
      });
    }
  }

  async function createPack(args: CreateWidgetPackArgs) {
    const pack = await invoke<WidgetPack>('create_widget_pack', { args });
    mutatePacks(packs => [...(packs ?? []), pack]);
    return pack;
  }

  async function createWidget(args: CreateWidgetArgs) {
    const configEntry = await invoke<WidgetConfig>(
      'create_widget_config',
      {
        args,
      },
    );

    mutatePacks(packs =>
      packs?.map(pack => {
        return pack.id === args.packId && pack.type === 'local'
          ? {
              ...pack,
              widgets: [...pack.widgets, configEntry],
            }
          : pack;
      }),
    );

    return configEntry;
  }

  async function deletePack(packId: string) {
    await invoke<void>('delete_widget_pack', { packId });
    mutatePacks(packs => packs?.filter(pack => pack.id !== packId));
  }

  async function updatePack(packId: string, args: UpdateWidgetPackArgs) {
    const updatedPack = await invoke<WidgetPack>('update_widget_pack', {
      packId,
      args,
    });

    mutatePacks(packs =>
      packs?.map(p => (p.id === packId ? updatedPack : p)),
    );

    return updatedPack;
  }

  async function deleteWidget(packId: string, widgetName: string) {
    await invoke<void>('delete_widget_config', { packId, widgetName });

    mutatePacks(packs =>
      packs?.map(pack => {
        return {
          ...pack,
          widgets: pack.widgets.filter(
            widgetConfig => widgetConfig.name !== widgetName,
          ),
        };
      }),
    );
  }

  const store: UserPacksContextState = {
    downloadedPacks,
    localPacks,
    allPacks,
    widgetStates,
    updateWidgetConfig,
    togglePreset,
    createPack,
    createWidget,
    updatePack,
    deletePack,
    deleteWidget,
  };

  return (
    <UserPacksContext.Provider value={store}>
      {props.children}
    </UserPacksContext.Provider>
  );
}

export function useUserPacks() {
  const context = useContext(UserPacksContext);

  if (!context) {
    throw new Error(
      '`useUserPacks` must be used within a `UserPacksProvider`.',
    );
  }

  return context;
}
