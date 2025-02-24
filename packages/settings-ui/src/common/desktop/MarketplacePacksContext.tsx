import {
  Accessor,
  createContext,
  createSignal,
  type JSX,
  Resource,
  useContext,
} from 'solid-js';
import { createResource } from 'solid-js';

import { WidgetConfig } from 'zebar';

export type MarketplaceWidgetPack = {
  id: string;
  name: string;
  author: string;
  previewImages: string[];
  versions: WidgetPackVersion[];
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

const marketplacePacksMock: MarketplaceWidgetPack[] = [
  {
    id: 'glzr-io.system-monitor',
    name: 'System Monitor',
    author: 'glzr-io',
    previewImages: [
      'https://placehold.co/200x200',
      'https://placehold.co/200x200',
    ],
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
    versions: [
      {
        versionNumber: '2.1.0',
        publishDate: new Date('2024-01-15'),
        commitSha: '8f62a3d',
        repoUrl: 'https://github.com/zebar-team/system-monitor',
        releaseNotes: 'Added dark mode support and new KPI cards',
      },
      {
        versionNumber: '2.0.1',
        publishDate: new Date('2023-12-20'),
        commitSha: '3e7b9c2',
        repoUrl: 'https://github.com/zebar-team/system-monitor',
        releaseNotes: 'Fixed responsive layout issues',
      },
      {
        versionNumber: '2.0.0',
        publishDate: new Date('2023-12-01'),
        commitSha: '1a2b3c4',
        repoUrl: 'https://github.com/zebar-team/system-monitor',
        releaseNotes: 'Major redesign and performance improvements',
      },
    ],
  },
  {
    id: 'glzr-io.weather-widgets',
    name: 'Weather Pack',
    author: 'glzr-io',
    previewImages: [
      'https://placehold.co/200x200',
      'https://placehold.co/200x200',
    ],
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
    versions: [
      {
        versionNumber: '2.1.0',
        publishDate: new Date('2024-01-15'),
        commitSha: '8f62a3d',
        repoUrl: 'https://github.com/weather-team/weather-widgets',
        releaseNotes: 'Added dark mode support and new KPI cards',
      },
      {
        versionNumber: '2.0.1',
        publishDate: new Date('2023-12-20'),
        commitSha: '3e7b9c2',
        repoUrl: 'https://github.com/weather-team/weather-widgets',
        releaseNotes: 'Fixed responsive layout issues',
      },
      {
        versionNumber: '2.0.0',
        publishDate: new Date('2023-12-01'),
        commitSha: '1a2b3c4',
        repoUrl: 'https://github.com/weather-team/weather-widgets',
        releaseNotes: 'Major redesign and performance improvements',
      },
    ],
  },
];

type MarketplacePacksContextState = {
  allPacks: Resource<MarketplaceWidgetPack[]>;
  selectedPack: Resource<MarketplaceWidgetPack>;
  previewPack: Accessor<MarketplaceWidgetPack | null>;
  install: (pack: MarketplaceWidgetPack) => void;
  selectPack: (packId: string) => void;
  startPreview: (pack: MarketplaceWidgetPack) => void;
  stopPreview: () => void;
};

const MarketplacePacksContext =
  createContext<MarketplacePacksContextState>();

// TODO: Remove once API calls are implemented.
function wait(timeout: number) {
  return new Promise(resolve => setTimeout(resolve, timeout));
}

export function MarketplacePacksProvider(props: {
  children: JSX.Element;
}) {
  // TODO: Fetch marketplace packs from the backend.
  const [allPacks] = createResource(
    async () => {
      await wait(2000);
      return marketplacePacksMock;
    },
    { initialValue: [] },
  );

  const [selectedPackId, setSelectedPackId] = createSignal<string | null>(
    null,
  );

  // TODO: Fetch marketplace pack from the backend.
  const [selectedPack] = createResource(
    () => selectedPackId() && allPacks(),
    async () => {
      await wait(2000);
      return allPacks().find(pack => pack.id === selectedPackId()) || null;
    },
  );

  const [previewPack, setPreviewPack] =
    createSignal<MarketplaceWidgetPack | null>(null);

  function selectPack(packId: string) {
    setSelectedPackId(packId);
  }

  function install(pack: MarketplaceWidgetPack) {
    // TODO
  }

  function startPreview(pack: MarketplaceWidgetPack) {
    setPreviewPack(pack);
  }

  function stopPreview() {
    setPreviewPack(null);
  }

  const store: MarketplacePacksContextState = {
    allPacks,
    selectedPack,
    previewPack,
    startPreview,
    stopPreview,
    install,
    selectPack,
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
