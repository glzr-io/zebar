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
    widgets: [
      {
        name: 'CPU Usage',
        htmlPath: 'cpu-usage.html',
      } as any as WidgetConfig,
      {
        name: 'Memory Usage',
        htmlPath: 'memory-usage.html',
      } as any as WidgetConfig,
      {
        name: 'Disk Space',
        htmlPath: 'disk-space.html',
      } as any as WidgetConfig,
    ],
    previewUrls: [],
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
    widgets: [
      {
        name: 'Current Weather',
        htmlPath: 'current-weather.html',
      } as any as WidgetConfig,
      {
        name: 'Weekly Forecast',
        htmlPath: 'weekly-forecast.html',
      } as any as WidgetConfig,
    ],
    previewUrls: [],
    excludeFiles: '',
  },
];

const localPacksMock = [
  {
    id: 'local.my-custom-widgets',
    name: 'My Custom Widgets',
    author: 'me',
    type: 'local' as const,
    description: 'Personal collection of widgets',
    version: '0.1.0',
    widgets: [
      {
        name: 'Todo List',
        htmlPath: 'todo-list.html',
      } as any as WidgetConfig,
    ],
    tags: ['todo', 'list', 'custom'],
    previewUrls: [],
    excludeFiles: '',
  },
];

export type WidgetPack = {
  id: string;
  name: string;
  author: string;
  type: 'local' | 'marketplace';
  previewUrls: string[];
  excludeFiles: string;
  versions?: WidgetPackVersion[];
  description: string;
  version: string;
  widgets: WidgetConfig[];
  tags: string[];
};

export type WidgetPackVersion = {
  versionNumber: string;
  releaseNotes: string;
  commitSha: string;
  repoUrl: string;
  publishDate: Date;
};

export type CreateWidgetPackForm = {
  name: string;
};

export type CreateWidgetArgs = {
  name: string;
  template: 'react-buildless' | 'solid-ts';
};

type UserPacksContextState = {
  communityPacks: Resource<WidgetPack[]>;
  localPacks: Resource<WidgetPack[]>;
  allPacks: Accessor<WidgetPack[]>;
  widgetConfigs: Resource<Record<string, WidgetConfig>>;
  widgetStates: Resource<Record<string, Widget>>;
  createPack: (pack: CreateWidgetPackForm) => Promise<void>;
  createWidget: (widget: CreateWidgetArgs) => Promise<void>;
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
  const [localPacks] = createResource(async () => localPacksMock);

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

  async function createPack(pack: CreateWidgetPackForm) {
    return invoke<void>('create_widget_pack', { pack });
  }

  async function createWidget(widget: CreateWidgetArgs) {
    return invoke<void>('create_widget', { widget });
  }

  async function deletePack(packId: string) {
    return invoke<void>('delete_widget_pack', { packId });
  }

  async function deleteWidget(widgetName: string) {
    return invoke<void>('delete_widget', { widgetName });
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
