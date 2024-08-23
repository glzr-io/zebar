import type { WindowConfig } from '~/user-config';

export interface WindowState {
  windowId: string;

  config: WindowConfig;

  configPath: string;
}
