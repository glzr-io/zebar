import type { Provider } from '../create-base-provider';

export interface AudioProviderConfig {
  type: 'audio';
}

export type AudioProvider = Provider<AudioProviderConfig, AudioOutput>;

export interface AudioDeviceInfo {
  deviceId: string;
  name: string;
  volume: number;
  isDefault: boolean;
}

export interface AudioOutput {
  devices: Record<string, AudioDeviceInfo>;
  defaultDevice: string | null;
}
