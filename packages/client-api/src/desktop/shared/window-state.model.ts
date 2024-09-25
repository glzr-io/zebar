import type { WidgetInstanceConfig } from '~/user-config';

export interface WindowState {
  windowId: string;

  config: WidgetInstanceConfig;

  configPath: string;
}
