import type { Owner } from 'solid-js';

import { createProviderListener } from '../create-provider-listener';
import type { ProviderType } from '../provider-type.model';

export interface IpProviderConfig {
  type: ProviderType.IP;

  refresh_interval: number;
}

export interface IpVariables {
  address: string;
  approxCity: string;
  approxCountry: string;
  approxLatitude: number;
  approxLongitude: number;
}

export async function createIpProvider(
  config: IpProviderConfig,
  owner: Owner,
) {
  const ipVariables = await createProviderListener<
    IpProviderConfig,
    IpVariables
  >(config, owner);

  return {
    get address() {
      return ipVariables().address;
    },
    get approxCity() {
      return ipVariables().approxCity;
    },
    get approxCountry() {
      return ipVariables().approxCountry;
    },
    get approxLatitude() {
      return ipVariables().approxLatitude;
    },
    get approxLongitude() {
      return ipVariables().approxLongitude;
    },
  };
}
