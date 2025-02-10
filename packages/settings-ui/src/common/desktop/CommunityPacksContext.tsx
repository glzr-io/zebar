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
  allPacks: Resource<WidgetPack[]>;
  selectedPack: Resource<WidgetPack>;
  previewPack: Accessor<WidgetPack | null>;
  install: (pack: WidgetPack) => void;
  selectPack: (packId: string) => void;
  startPreview: (pack: WidgetPack) => void;
  stopPreview: () => void;
};

const CommunityPacksContext = createContext<CommunityPacksContextState>();

export function CommunityPacksProvider(props: { children: JSX.Element }) {
  // TODO: Fetch community packs from the backend.
  const [allPacks] = createResource(async () => communityPacksMock, {
    initialValue: [],
  });

  const [selectedPackId, setSelectedPackId] = createSignal<string | null>(
    null,
  );

  // TODO: Fetch community pack from the backend.
  const [selectedPack] = createResource(async () => {
    const packId = selectedPackId();
    return allPacks().find(pack => pack.id === packId) || null;
  });

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
