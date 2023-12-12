import { HostProviderConfig } from '~/user-config';
import { memoize } from '~/utils';
import { createProviderListener } from '../create-provider-listener';

export interface HostVariables {
  isLoading: boolean;
  hostname: string | null;
  osName: string | null;
  osVersion: string | null;
  friendlyOsVersion: string | null;
  bootTime: number;
  uptime: number;
}

export const createHostProvider = memoize((config: HostProviderConfig) => {
  const [hostVariables] = createProviderListener<
    HostProviderConfig,
    HostVariables
  >(config);

  return {
    get isLoading() {
      return hostVariables()?.isLoading ?? true;
    },
    get hostname() {
      return hostVariables()?.hostname;
    },
    get osName() {
      return hostVariables()?.osName;
    },
    get osVersion() {
      return hostVariables()?.osVersion;
    },
    get friendlyOsVersion() {
      return hostVariables()?.friendlyOsVersion;
    },
    get bootTime() {
      return hostVariables()?.bootTime;
    },
    get uptime() {
      return hostVariables()?.uptime;
    },
  };
});
