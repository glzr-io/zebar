import type { Provider } from '../create-base-provider';

export interface KeyboardProviderConfig {
  type: 'keyboard';

  /**
   * How often this provider refreshes in milliseconds.
   */
  refreshInterval?: number;
}

export type KeyboardProvider = Provider<
  KeyboardProviderConfig,
  KeyboardOutput
>;

export interface KeyboardOutput {
  layout: string;
}
