import type { Provider } from '../create-base-provider';

export interface MediaProviderConfig {
  type: 'media';
}

export interface MediaOutput {
  session: MediaSession | null;
  play(): void;
  pause(): void;
  togglePlayPause(): void;
  next(): void;
  previous(): void;
}

export interface MediaSession {
  title: string;
  artist: string | null;
  albumTitle: string | null;
  albumArtist: string | null;
  trackNumber: number;
  startTime: number;
  endTime: number;
  position: number;
  isPlaying: boolean;
}

export type MediaProvider = Provider<MediaProviderConfig, MediaOutput>;
