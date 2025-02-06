import type { Provider } from '../create-base-provider';

export interface FocusedWindowProviderConfig {
  type: 'focusedWindow';
}

export interface FocusedWindowOutput {
  title: string;
  iconBytes: number[];
  iconBlob: Blob;
  iconURL: string;
}

export type FocusedWindowProvider = Provider<
  FocusedWindowProviderConfig,
  FocusedWindowOutput
>;
