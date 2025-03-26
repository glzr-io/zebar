import type { WidgetConfig } from './widget-config';

export type WidgetPack =
  | {
      type: 'marketplace';
      id: string;
      name: string;
      previewImages: string[];
      excludeFiles: string;
      directoryPath: string;
      description: string;
      version: string;
      widgets: WidgetConfig[];
      tags: string[];
      metadata: {
        packId: string;
        installedAt: number;
        version: string;
      };
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
