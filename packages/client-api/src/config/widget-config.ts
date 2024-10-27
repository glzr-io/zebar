import type { WidgetPreset } from './widget-preset';

export type WidgetConfig = {
  htmlPath: string;
  zOrder: 'normal' | 'top_most' | 'bottom_most';
  shownInTaskbar: boolean;
  focused: boolean;
  resizable: boolean;
  transparent: boolean;
  presets: WidgetPreset[];
};
