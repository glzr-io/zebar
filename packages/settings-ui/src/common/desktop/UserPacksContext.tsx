import {
  createContext,
  createMemo,
  type Accessor,
  type JSX,
  Resource,
  useContext,
} from 'solid-js';
import { invoke } from '@tauri-apps/api/core';
import { listen, type Event } from '@tauri-apps/api/event';
import { createResource } from 'solid-js';
import type { Widget, WidgetConfig } from 'zebar';

const downloadedPacksMock: WidgetPack[] = [
  {
    id: 'glzr-io.system-monitor',
    name: 'System Monitor',
    author: 'glzr-io',
    type: 'marketplace' as const,
    directoryPath: 'C:\\Users\\larsb\\.glzr\\zebar\\fdsafdsafdsa',
    description: 'CPU, memory, and disk usage widgets',
    version: '1.0.0',
    tags: ['system', 'monitor', 'cpu', 'memory', 'disk'],
    widgets: [
      {
        name: 'cpu-usage',
        htmlPath: 'cpu-usage.html',
      } as any as WidgetConfig,
      {
        name: 'memory-usage',
        htmlPath: 'memory-usage.html',
      } as any as WidgetConfig,
      {
        name: 'disk-space',
        htmlPath: 'disk-space.html',
      } as any as WidgetConfig,
    ],
    previewImages: [],
    excludeFiles: '',
  },
  {
    id: 'glzr-io.weather-widgets',
    name: 'Weather Pack',
    author: 'glzr-io',
    type: 'marketplace' as const,
    directoryPath: 'C:\\Users\\larsb\\.glzr\\zebar\\fdsafdsafdsa',
    description: 'Current weather and forecast widgets',
    version: '2.1.0',
    tags: ['weather', 'forecast', 'current'],
    widgets: [
      {
        name: 'current-weather',
        htmlPath: 'current-weather.html',
      } as any as WidgetConfig,
      {
        name: 'weekly-forecast',
        htmlPath: 'weekly-forecast.html',
      } as any as WidgetConfig,
    ],
    previewImages: [],
    excludeFiles: '',
  },
];

export type WidgetPack =
  | {
      type: 'marketplace';
      id: string;
      name: string;
      author: string;
      previewImages: string[];
      excludeFiles: string;
      directoryPath: string;
      description: string;
      version: string;
      widgets: WidgetConfig[];
      tags: string[];
    }
  | {
      type: 'local';
      id: string;
      name: string;
      previewImages: string[];
      excludeFiles: string;
      directoryPath: string;
      description: string;
      widgets: WidgetConfig[];
      tags: string[];
    };

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
  widgetPaths?: string[];
};

export type CreateWidgetArgs = {
  name: string;
  packId: string;
  template: 'react_buildless' | 'solid_typescript';
};

type UserPacksContextState = {
  downloadedPacks: Resource<WidgetPack[]>;
  localPacks: Resource<WidgetPack[]>;
  allPacks: Accessor<WidgetPack[]>;
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
  // TODO: Fetch installed marketplace packs from the backend.
  const [downloadedPacks] = createResource(
    async () => downloadedPacksMock,
  );

  const [localPacks, { mutate: mutateLocalPacks }] = createResource(
    async () => invoke<WidgetPack[]>('widget_packs'),
  );

  const allPacks = createMemo(() => [
    ...(downloadedPacks() ?? []),
    ...(localPacks() ?? []),
  ]);

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

    mutateLocalPacks(packs =>
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
      });
    } else {
      await invoke<void>('start_widget_preset', {
        packId,
        widgetName,
        presetName,
      });
    }
  }

  async function createPack(args: CreateWidgetPackArgs) {
    const pack = await invoke<WidgetPack>('create_widget_pack', { args });
    mutateLocalPacks(packs => [...(packs ?? []), pack]);
    return pack;
  }

  async function createWidget(args: CreateWidgetArgs) {
    const configEntry = await invoke<WidgetConfig>(
      'create_widget_config',
      {
        args,
      },
    );

    mutateLocalPacks(packs =>
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
    mutateLocalPacks(packs => packs?.filter(pack => pack.id !== packId));
  }

  async function updatePack(packId: string, args: UpdateWidgetPackArgs) {
    const updatedPack = await invoke<WidgetPack>('update_widget_pack', {
      packId,
      args,
    });

    mutateLocalPacks(packs =>
      packs?.map(p => (p.id === packId ? updatedPack : p)),
    );

    return updatedPack;
  }

  async function deleteWidget(packId: string, widgetName: string) {
    await invoke<void>('delete_widget_config', { packId, widgetName });

    mutateLocalPacks(packs =>
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
