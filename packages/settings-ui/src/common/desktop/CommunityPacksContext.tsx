import { createContext, type JSX, Resource, useContext } from 'solid-js';
import { createResource } from 'solid-js';

import { WidgetPack } from './UserPacksContext';

const communityPacksMock = [
  {
    id: 'system-monitor',
    name: 'System Monitor',
    author: 'Zebar Team',
    galleryUrls: [
      'https://placehold.co/200x200',
      'https://placehold.co/200x200',
    ],
    description: 'CPU, memory, and disk usage widgets',
    version: '1.0.0',
    license: 'MIT',
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
    galleryUrls: [
      'https://placehold.co/200x200',
      'https://placehold.co/200x200',
    ],
    description: 'Current weather and forecast widgets',
    version: '2.1.0',
    license: 'MIT',
    tags: ['weather', 'forecast', 'current'],
    widgets: [
      { id: 'current-weather', name: 'Current Weather' },
      { id: 'forecast', name: 'Weekly Forecast' },
    ],
  },
];

type CommunityPacksContextState = {
  all: Resource<WidgetPack[]>;
};

const CommunityPacksContext = createContext<CommunityPacksContextState>();

export function CommunityPacksProvider(props: { children: JSX.Element }) {
  // TODO: Fetch community packs from the backend.
  const [all] = createResource(async () => communityPacksMock, {
    initialValue: [],
  });

  const store: CommunityPacksContextState = {
    all,
  };

  return (
    <CommunityPacksContext.Provider value={store}>
      {props.children}
    </CommunityPacksContext.Provider>
  );
}

export function useCommunityPacks() {
  const context = useContext(CommunityPacksContext);

  if (!context) {
    throw new Error(
      '`useCommunityPacks` must be used within a `CommunityPacksProvider`.',
    );
  }

  return context;
}
