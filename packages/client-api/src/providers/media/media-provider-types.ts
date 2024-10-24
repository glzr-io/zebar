import type {Provider} from '../create-base-provider';

export interface MediaProviderConfig {
    type: 'media';
    /**
     * How often this provider refreshes in milliseconds.
     */
    refreshInterval?: number;
}

export interface MediaOutput {
    title: string;
    subTitle: string;
    trackNumber: number;
    artist: string;
    albumTitle: string;
    isPlaying: boolean;
    isSpotify: boolean;
}

export type MediaProvider = Provider<MediaProviderConfig, MediaOutput>;
