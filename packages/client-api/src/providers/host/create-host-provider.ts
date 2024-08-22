import type { Owner } from 'solid-js';

import { createProviderListener } from '../create-provider-listener';

export interface HostProviderConfig {
  type: 'host';

  /**
   * How often this provider refreshes in milliseconds.
   */
  refreshInterval?: number;
}

export interface HostProvider {
  hostname: string | null;
  osName: string | null;
  osVersion: string | null;
  friendlyOsVersion: string | null;
  bootTime: number;
  uptime: number;
}

export async function createHostProvider(
  config: HostProviderConfig,
  owner: Owner,
) {
  const hostVariables = await createProviderListener<
    HostProviderConfig,
    HostProvider
  >(config, owner);

  return {
    get hostname() {
      return hostVariables().hostname;
    },
    get osName() {
      return hostVariables().osName;
    },
    get osVersion() {
      return hostVariables().osVersion;
    },
    get friendlyOsVersion() {
      return hostVariables().friendlyOsVersion;
    },
    get bootTime() {
      return hostVariables().bootTime;
    },
    get uptime() {
      return hostVariables().uptime;
    },
  };
}
