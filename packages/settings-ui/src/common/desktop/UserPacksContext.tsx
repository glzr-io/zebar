import { createContext, type JSX, Resource, useContext } from 'solid-js';
import { invoke } from '@tauri-apps/api/core';
import { listen, type Event } from '@tauri-apps/api/event';
import { createResource } from 'solid-js';
import type { Widget, WidgetConfig } from 'zebar';

const communityPacksMock = [
  {
    id: 'system-monitor',
    name: 'System Monitor',
    author: 'Zebar Team',
    description: 'CPU, memory, and disk usage widgets',
    version: '1.0.0',
    tags: ['system', 'monitor', 'cpu', 'memory', 'disk'],
    widgets: [
      { id: 'cpu-usage', name: 'CPU Usage' },
      { id: 'memory-usage', name: 'Memory Usage' },
      { id: 'disk-space', name: 'Disk Space' },
    ],
  },
  {
    id: 'weather-widgets',
    name: 'Weather Pack',
    author: 'Weather Team',
    description: 'Current weather and forecast widgets',
    version: '2.1.0',
    tags: ['weather', 'forecast', 'current'],
    widgets: [
      { id: 'current-weather', name: 'Current Weather' },
      { id: 'forecast', name: 'Weekly Forecast' },
    ],
  },
];

const localPacksMock = [
  {
    id: 'my-custom-widgets',
    name: 'My Custom Widgets',
    author: 'me',
    description: 'Personal collection of widgets',
    version: '0.1.0',
    widgets: [{ id: 'todo-list', name: 'Todo List' }],
    tags: ['todo', 'list', 'custom'],
  },
];

export type WidgetPack = {
  id: string;
  name: string;
  author: string;
  galleryUrls?: string[];
  versions?: WidgetPackVersion[];
  description: string;
  version: string;
  widgets: { id: string; name: string }[];
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

type UserPacksContextState = {
  communityPacks: Resource<WidgetPack[]>;
  localPacks: Resource<WidgetPack[]>;
  widgetConfigs: Resource<Record<string, WidgetConfig>>;
  widgetStates: Resource<Record<string, Widget>>;
  createPack: (pack: CreateWidgetPackForm) => void;
  deletePack: (packId: string) => void;
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
    // TODO
  }

  async function deletePack(packId: string) {
    // TODO
  }

  const store: UserPacksContextState = {
    communityPacks,
    localPacks,
    widgetConfigs,
    widgetStates,
    updateWidgetConfig,
    togglePreset,
    createPack,
    deletePack,
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
