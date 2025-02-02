import { invoke } from '@tauri-apps/api/core';
import type { Provider } from '../create-base-provider';

export interface SystrayProviderConfig {
  type: 'systray';
}

export type SystrayProvider = Provider<
  SystrayProviderConfig,
  SystrayOutput
>;

export interface SystrayOutput {
//   icons: SystrayIcon[];
  icons: TempSystrayIcon[];
}

export interface SystrayIcon {
  id: string;
  icon: string;
  title: string;
  onRightClick: () => Promise<void>;
  onLeftClick: () => Promise<void>;
}

// TODO - this is for testing
export class TempSystrayIcon implements SystrayIcon {
  id: string;
  icon: string;
  title: string;

  constructor(id: string, icon: string, title: string) {
    this.id = id;
    this.icon = icon; 
    this.title = title;
  }

  async onRightClick(): Promise<void> {
    await invoke('systray_right_click', { iconId: this.id });
  }

  async onLeftClick(): Promise<void> {
    await invoke('systray_left_click', { iconId: this.id });
  }
}
