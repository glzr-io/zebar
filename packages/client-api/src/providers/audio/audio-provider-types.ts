import type { Provider } from '../create-base-provider';

export interface AudioProviderConfig {
  type: 'audio';
}

export type AudioProvider = Provider<AudioProviderConfig, AudioOutput>;

export interface AudioOutput {
  defaultPlaybackDevice: AudioDevice | null;
  defaultRecordingDevice: AudioDevice | null;
  playbackDevices: AudioDevice[];
  recordingDevices: AudioDevice[];
  setVolume(volume: number, options?: SetVolumeOptions): Promise<void>;
  setMute(mute: boolean): Promise<void>;
}

export interface SetVolumeOptions {
  deviceId?: string;
}

export interface AudioDevice {
  deviceId: string;
  name: string;
  volume: number;
  type: AudioDeviceType;
  isDefaultPlayback: boolean;
  isDefaultRecording: boolean;
  isMuted: boolean;
}

export type AudioDeviceType = 'playback' | 'recording';
