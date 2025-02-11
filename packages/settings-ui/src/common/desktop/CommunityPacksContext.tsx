import {
  Accessor,
  createContext,
  createMemo,
  createSignal,
  type JSX,
  Resource,
  useContext,
} from 'solid-js';
import { createResource } from 'solid-js';

import { WidgetPack } from './UserPacksContext';
import { useParams } from '@solidjs/router';

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
    tags: ['system', 'monitor', 'cpu', 'memory', 'disk'],
    widgets: [
      { id: 'cpu-usage', name: 'CPU Usage' },
      { id: 'memory-usage', name: 'Memory Usage' },
      { id: 'disk-space', name: 'Disk Space' },
    ],
    versions: [
      {
        versionNumber: '2.1.0',
        publishDate: new Date('2024-01-15'),
        commitHash: '8f62a3d',
        repoUrl: 'https://github.com/zebar-team/system-monitor',
        releaseNotes: 'Added dark mode support and new KPI cards',
      },
      {
        versionNumber: '2.0.1',
        publishDate: new Date('2023-12-20'),
        commitHash: '3e7b9c2',
        repoUrl: 'https://github.com/zebar-team/system-monitor',
        releaseNotes: 'Fixed responsive layout issues',
      },
      {
        versionNumber: '2.0.0',
        publishDate: new Date('2023-12-01'),
        commitHash: '1a2b3c4',
        repoUrl: 'https://github.com/zebar-team/system-monitor',
        releaseNotes: 'Major redesign and performance improvements',
      },
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
    tags: ['weather', 'forecast', 'current'],
    widgets: [
      { id: 'current-weather', name: 'Current Weather' },
      { id: 'forecast', name: 'Weekly Forecast' },
    ],
    versions: [
      {
        versionNumber: '2.1.0',
        publishDate: new Date('2024-01-15'),
        commitHash: '8f62a3d',
        repoUrl: 'https://github.com/weather-team/weather-widgets',
        releaseNotes: 'Added dark mode support and new KPI cards',
      },
      {
        versionNumber: '2.0.1',
        publishDate: new Date('2023-12-20'),
        commitHash: '3e7b9c2',
        repoUrl: 'https://github.com/weather-team/weather-widgets',
        releaseNotes: 'Fixed responsive layout issues',
      },
      {
        versionNumber: '2.0.0',
        publishDate: new Date('2023-12-01'),
        commitHash: '1a2b3c4',
        repoUrl: 'https://github.com/weather-team/weather-widgets',
        releaseNotes: 'Major redesign and performance improvements',
      },
    ],
  },
];

type CommunityPacksContextState = {
  allPacks: Resource<WidgetPack[]>;
  selectedPack: Resource<WidgetPack>;
  previewPack: Accessor<WidgetPack | null>;
  install: (pack: WidgetPack) => void;
  selectPack: (packId: string) => void;
  startPreview: (pack: WidgetPack) => void;
  stopPreview: () => void;
};

const CommunityPacksContext = createContext<CommunityPacksContextState>();

// TODO: Remove once API calls are implemented.
function wait(timeout: number) {
  return new Promise(resolve => setTimeout(resolve, timeout));
}

export function CommunityPacksProvider(props: { children: JSX.Element }) {
  // TODO: Fetch community packs from the backend.
  const [allPacks] = createResource(
    async () => {
      await wait(2000);
      return communityPacksMock;
    },
    { initialValue: [] },
  );

  const [selectedPackId, setSelectedPackId] = createSignal<string | null>(
    null,
  );

  // TODO: Fetch community pack from the backend.
  const [selectedPack] = createResource(
    () => selectedPackId() && allPacks(),
    async () => {
      await wait(2000);
      return allPacks().find(pack => pack.id === selectedPackId()) || null;
    },
  );

  const [previewPack, setPreviewPack] = createSignal<WidgetPack | null>(
    null,
  );

  function selectPack(packId: string) {
    setSelectedPackId(packId);
  }

  function install(pack: WidgetPack) {
    // TODO
  }

  function startPreview(pack: WidgetPack) {
    setPreviewPack(pack);
  }

  function stopPreview() {
    setPreviewPack(null);
  }

  const store: CommunityPacksContextState = {
    allPacks,
    selectedPack,
    previewPack,
    startPreview,
    stopPreview,
    install,
    selectPack,
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
