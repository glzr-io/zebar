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

const communityPacksMock = [
  {
    id: 'glzr-io.system-monitor',
    name: 'System Monitor',
    author: 'glzr-io',
    type: 'marketplace' as const,
    description: 'CPU, memory, and disk usage widgets',
    version: '1.0.0',
    tags: ['system', 'monitor', 'cpu', 'memory', 'disk'],
    widgetConfigs: [
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
    description: 'Current weather and forecast widgets',
    version: '2.1.0',
    tags: ['weather', 'forecast', 'current'],
    widgetConfigs: [
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

const localPacksMock = [
  {
    id: 'local.my-custom-widgets',
    name: 'My Custom Widgets',
    author: 'me',
    type: 'local' as 'local' | 'marketplace',
    description: 'Personal collection of widgets',
    version: '0.1.0',
    widgetConfigs: [
      {
        name: 'todo-list',
        htmlPath: 'todo-list.html',
      } as any as WidgetConfig,
    ],
    tags: ['todo', 'list', 'custom'],
    previewImages: [],
    excludeFiles: '',
  },
];

export type WidgetPack = {
  id: string;
  name: string;
  author: string;
  type: 'local' | 'marketplace';
  previewImages: string[];
  excludeFiles: string;
  versions?: WidgetPackVersion[];
  description: string;
  version: string;
  widgetConfigs: WidgetConfig[];
  tags: string[];
};

export type WidgetPackVersion = {
  versionNumber: string;
  releaseNotes: string;
  commitSha: string;
  repoUrl: string;
  publishDate: Date;
};

export type CreateWidgetPackArgs = {
  name: string;
  description: string;
  tags: string[];
  previewImages: string[];
  excludeFiles: string;
  widgets: CreateWidgetArgs[];
};

export type CreateWidgetArgs = {
  name: string;
  packId: string;
  template: 'react-buildless' | 'solid-ts';
};

type UserPacksContextState = {
  communityPacks: Resource<WidgetPack[]>;
  localPacks: Resource<WidgetPack[]>;
  allPacks: Accessor<WidgetPack[]>;
  widgetConfigs: Resource<Record<string, WidgetConfig>>;
  widgetStates: Resource<Record<string, Widget>>;
  createPack: (args: CreateWidgetPackArgs) => Promise<WidgetPack>;
  createWidget: (args: CreateWidgetArgs) => Promise<WidgetConfig>;
  deletePack: (packId: string) => Promise<void>;
  deleteWidget: (widgetName: string) => Promise<void>;
  updateWidgetConfig: (
    configPath: string,
    newConfig: WidgetConfig,
  ) => Promise<void>;
  togglePreset: (configPath: string, presetName: string) => Promise<void>;
};

const UserPacksContext = createContext<UserPacksContextState>();

export function UserPacksProvider(props: { children: JSX.Element }) {
  // TODO: Fetch installed community packs from the backend.
  const [communityPacks] = createResource(async () => communityPacksMock);

  // TODO: Fetch local packs from the backend.
  const [localPacks, { mutate: mutateLocalPacks }] = createResource(
    async () => localPacksMock,
  );

  const allPacks = createMemo(() => [
    ...(communityPacks() ?? []),
    ...(localPacks() ?? []),
  ]);

  const [widgetConfigs, { mutate: mutateWidgetConfigs }] = createResource(
    async () => invoke<Record<string, WidgetConfig>>('widget_configs'),
    { initialValue: {} },
  );

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
    configPath: string,
    newConfig: WidgetConfig,
  ) {
    mutateWidgetConfigs(configs => ({
      ...configs,
      [configPath]: newConfig,
    }));

    await invoke<void>('update_widget_config', {
      configPath,
      newConfig,
    });
  }

  async function togglePreset(configPath: string, presetName: string) {
    const states = widgetStates();

    const configStates = Object.values(states).filter(
      state => state.configPath === configPath,
    );

    const presetStates = configStates.filter(
      // @ts-ignore - TODO
      state => state.openOptions?.preset === presetName,
    );

    if (presetStates.length > 0) {
      await invoke<void>('stop_preset', {
        configPath,
        presetName,
      });
    } else {
      await invoke<void>('start_preset', {
        configPath,
        presetName,
      });
    }
  }

  async function createPack(args: CreateWidgetPackArgs) {
    const pack = await invoke<WidgetPack>('create_widget_pack', { args });
    mutateLocalPacks(packs => [...packs, pack]);
    return pack;
  }

  async function createWidget(args: CreateWidgetArgs) {
    const widget = await invoke<WidgetConfig>('create_widget', { args });

    mutateLocalPacks(packs =>
      packs.map(pack => {
        return pack.id === args.packId
          ? { ...pack, widgetConfigs: [...pack.widgetConfigs, widget] }
          : pack;
      }),
    );

    return widget;
  }

  async function deletePack(packId: string) {
    await invoke<void>('delete_widget_pack', { packId });
    mutateLocalPacks(packs => packs.filter(pack => pack.id !== packId));
  }

  async function deleteWidget(widgetName: string) {
    await invoke<void>('delete_widget', { widgetName });

    mutateLocalPacks(packs =>
      packs.map(pack => {
        return {
          ...pack,
          widgetConfigs: pack.widgetConfigs.filter(
            w => w.name !== widgetName,
          ),
        };
      }),
    );
  }

  const store: UserPacksContextState = {
    communityPacks,
    localPacks,
    allPacks,
    widgetConfigs,
    widgetStates,
    updateWidgetConfig,
    togglePreset,
    createPack,
    createWidget,
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
