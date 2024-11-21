import type { Provider } from '../create-base-provider';

export interface MediaProviderConfig {
  type: 'media';
}

export interface MediaOutput {
  /** @deprecated Use {@link currentSession} instead */
  session: MediaSession | null;
  currentSession: MediaSession | null;
  allSessions: MediaSession[];
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
  isCurrentSession: boolean;
}

export type MediaProvider = Provider<MediaProviderConfig, MediaOutput>;
