import type { Provider } from '../create-base-provider';

export interface MediaProviderConfig {
  type: 'media';
}

export interface MediaOutput {
  session: MediaSession;
}

export interface MediaSession {
  title: string;
  artist: string;
  albumTitle: string;
  albumArtist: string;
  trackNumber: number;
  startTime: number;
  endTime: number;
  position: number;
  isPlaying: boolean;
}

export type MediaProvider = Provider<MediaProviderConfig, MediaOutput>;
