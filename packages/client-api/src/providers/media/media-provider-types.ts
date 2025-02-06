import type { Provider } from '../create-base-provider';

export interface MediaProviderConfig {
  type: 'media';
}

export interface MediaOutput {
  /** @deprecated Use {@link currentSession} instead */
  session: MediaSession | null;
  currentSession: MediaSession | null;
  allSessions: MediaSession[];
  play(options?: MediaControlOptions): void;
  pause(options?: MediaControlOptions): void;
  togglePlayPause(options?: MediaControlOptions): void;
  next(options?: MediaControlOptions): void;
  previous(options?: MediaControlOptions): void;
}

export interface MediaControlOptions {
  sessionId?: string;
}

export interface MediaSession {
  sessionId: string;
  title: string | null;
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
