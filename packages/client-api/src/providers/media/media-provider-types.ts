import type { Provider } from '../create-base-provider';

export interface MediaProviderConfig {
  type: 'media';
}

export interface MediaOutput {
  title: string;
  artist: string;
  album: string;
  album_artist: string;
  track_number: number;
  start_time: number;
  end_time: number;
  duration: number;
  isPlaying: boolean;
}

export type MediaProvider = Provider<MediaProviderConfig, MediaOutput>;
