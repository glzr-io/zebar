import { Owner } from 'solid-js';

import { IpProviderConfig } from '~/user-config';
import { createProviderListener } from '../create-provider-listener';

export interface IpVariables {
  address: string;
  approxCity: string;
  approxCountry: string;
  approxLatitude: number;
  approxLongitude: number;
}

export async function createIpProvider(config: IpProviderConfig, owner: Owner) {
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
