import type { Provider } from '../create-base-provider';

export interface SystrayProviderConfig {
  type: 'systray';
}

export type SystrayProvider = Provider<
  SystrayProviderConfig,
  SystrayOutput
>;

export interface SystrayOutput {
  icons: SystrayIcon[];
  onHoverEnter: (iconId: string) => Promise<void>;
  onHoverLeave: (iconId: string) => Promise<void>;
  onHoverMove: (iconId: string) => Promise<void>;
  onRightClick: (iconId: string) => Promise<void>;
  onLeftClick: (iconId: string) => Promise<void>;
  onMiddleClick: (iconId: string) => Promise<void>;
}

export interface SystrayIcon {
  id: string;
  iconBytes: number[];
  iconBlob: Blob;
  iconUrl: string;
  tooltip: string;
}
