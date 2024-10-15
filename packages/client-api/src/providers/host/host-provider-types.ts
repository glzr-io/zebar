import type { Provider } from '../create-base-provider';

export interface HostProviderConfig {
  type: 'host';

  /**
   * How often this provider refreshes in milliseconds.
   */
  refreshInterval?: number;
}

export type HostProvider = Provider<HostProviderConfig, HostOutput>;

export interface HostOutput {
  hostname: string | null;
  osName: string | null;
  osVersion: string | null;
  friendlyOsVersion: string | null;
  bootTime: number;
  uptime: number;
}
