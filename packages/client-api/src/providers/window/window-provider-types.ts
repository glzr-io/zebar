import type { Provider } from '../create-base-provider';

export interface WindowProviderConfig {
  type: 'window';

  /**
   * How often this provider refreshes in milliseconds.
   */
  refreshInterval?: number;  
}

export type WindowProvider = Provider<WindowProviderConfig, WindowOutput>;

export interface WindowOutput {
  title: string;
}