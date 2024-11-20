import type { Provider } from '../create-base-provider';

export interface AudioProviderConfig {
  type: 'audio';
}

export type AudioProvider = Provider<AudioProviderConfig, AudioOutput>;

export interface AudioOutput {
  defaultDevice: AudioDevice;
  devices: AudioDevice[];
}

export interface AudioDevice {
  deviceId: string;
  name: string;
  volume: number;
  isDefault: boolean;
}
