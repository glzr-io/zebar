import type { WidgetCaching } from './widget-caching';
import type { WidgetPreset } from './widget-preset';
import type { WidgetPrivileges } from './widget-privileges';

export type WidgetConfig = {
  name: string;
  htmlPath: string;
  zOrder: 'normal' | 'top_most' | 'bottom_most';
  shownInTaskbar: boolean;
  focused: boolean;
  resizable: boolean;
  transparent: boolean;
  caching: WidgetCaching;
  privileges: WidgetPrivileges;
  presets: WidgetPreset[];
};
