import type { WidgetConfig } from './widget-config';

export type WidgetPack =
  | {
      type: 'marketplace';
      id: string;
      name: string;
      version: string;
      previewImages: string[];
      repositoryUrl: string;
      directoryPath: string;
      description: string;
      widgets: WidgetConfig[];
      tags: string[];
      metadata: {
        packId: string;
        installedAt: number;
        version: string;
      };
    }
  | {
      type: 'custom';
      id: string;
      name: string;
      version: string;
      previewImages: string[];
      repositoryUrl: string;
      directoryPath: string;
      description: string;
      widgets: WidgetConfig[];
      tags: string[];
    };
