import type { WidgetConfig } from '~/user-config';

export interface WindowState {
  windowId: string;

  config: WidgetConfig;

  configPath: string;
}
