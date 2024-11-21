import type { Provider } from '../create-base-provider';

export interface MediaProviderConfig {
  type: 'media';
}

export interface MediaOutput {
  /** @deprecated Use {@link currentSession} instead */
  session: MediaSession | null;
  currentSession: MediaSession | null;
  allSessions: MediaSession[];
  play(args?: MediaControlArgs): void;
  pause(args?: MediaControlArgs): void;
  togglePlayPause(args?: MediaControlArgs): void;
  next(args?: MediaControlArgs): void;
  previous(args?: MediaControlArgs): void;
}

export interface MediaControlArgs {
  sessionId: string;
}

export interface MediaSession {
  sessionId: string;
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
