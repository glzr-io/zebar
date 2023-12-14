import axios from 'axios';
import { Owner, onCleanup, runWithOwner } from 'solid-js';
import { createStore } from 'solid-js/store';

import { IpProviderConfig } from '~/user-config';
import { IpInfoApiResponse } from './ip-info-api-response.model';

export interface IpVariables {
  address: string;
  approxCity: string;
  approxCountry: string;
  approxLatitude: string;
  approxLongitude: string;
}

export async function createIpProvider(config: IpProviderConfig, owner: Owner) {
  const [ipVariables, setIpVariables] = createStore<IpVariables>({
    address: '',
    approxCity: '',
    approxCountry: '',
    approxLatitude: '',
    approxLongitude: '',
  });

  await refresh();
  const interval = setInterval(refresh, config.refresh_interval_ms);
  runWithOwner(owner, () => onCleanup(() => clearInterval(interval)));

  async function refresh() {
    // Use https://ipinfo.io as provider for IP-related info.
    const { data } = await axios.get<IpInfoApiResponse>(
      'https://ipinfo.io/json',
    );

    setIpVariables({
      address: data.ip,
      approxCity: data.city,
      approxCountry: data.country,
      approxLatitude: data.loc.split(',')[0],
      approxLongitude: data.loc.split(',')[1],
    });
  }

  return {
    get address() {
      return ipVariables.address;
    },
    get approxCity() {
      return ipVariables.approxCity;
    },
    get approxCountry() {
      return ipVariables.approxCountry;
    },
    get approxLatitude() {
      return ipVariables.approxLatitude;
    },
    get approxLongitude() {
      return ipVariables.approxLongitude;
    },
    refresh,
  };
}
