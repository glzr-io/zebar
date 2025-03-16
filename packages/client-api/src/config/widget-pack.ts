import type { WidgetConfig } from './widget-config';

export type WidgetPack = {
  name: string;
  description: string;
  tags: string[];
  previewImages: string[];
  excludeFiles: string[];
  widgets: WidgetConfig[];
};
