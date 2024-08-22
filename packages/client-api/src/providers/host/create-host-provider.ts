import type { Owner } from 'solid-js';

import { createProviderListener } from '../create-provider-listener';
import type { ProviderType } from '../provider-type.model';

export interface HostProviderConfig {
  type: ProviderType.HOST;

  refresh_interval: number;
}

export interface HostVariables {
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
    HostVariables
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
