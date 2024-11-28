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
}

export interface AudioDevice {
  id: string;
  name: string;
  volume: number | null;
  roles: AudioDeviceRole[];
  type: AudioDeviceType;
  isDefaultPlayback: boolean;
  isDefaultRecording: boolean;
}

export type AudioDeviceRole = 'multimedia' | 'communications' | 'console';
export type AudioDeviceType = 'playback' | 'recording' | 'hybrid';
