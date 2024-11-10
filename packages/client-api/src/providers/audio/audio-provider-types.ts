import type { Provider } from '../create-base-provider';

export interface AudioProviderConfig {
  type: 'audio';
}

export type AudioProvider = Provider<AudioProviderConfig, AudioOutput>;

export interface AudioOutput {
  volume: number;
  currentDevice: string;
}

