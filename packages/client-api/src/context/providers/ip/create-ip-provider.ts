import axios from 'axios';
import { onCleanup } from 'solid-js';
import { createStore } from 'solid-js/store';

import { memoize } from '~/utils';
import { IpProviderConfig } from '~/user-config';
import { IpInfoApiResponse } from './ip-info-api-response.model';

export interface IpVariables {
  isLoading: boolean;
  address: string;
  approxCity: string;
  approxCountry: string;
  approxLatitude: string;
  approxLongitude: string;
}

export const createIpProvider = memoize((config: IpProviderConfig) => {
  const [ipVariables, setIpVariables] = createStore<IpVariables>({
    isLoading: true,
    address: '',
    approxCity: '',
    approxCountry: '',
    approxLatitude: '',
    approxLongitude: '',
  });

  refresh();
  const interval = setInterval(() => refresh(), config.refresh_interval_ms);
  onCleanup(() => clearInterval(interval));

  async function refresh() {
    // Use https://ipinfo.io as provider for IP-related info.
    const { data } = await axios.get<IpInfoApiResponse>(
      'https://ipinfo.io/json',
    );

    setIpVariables({
      isLoading: false,
      address: data.ip,
      approxCity: data.city,
      approxCountry: data.country,
      approxLatitude: data.loc.split(',')[0],
      approxLongitude: data.loc.split(',')[1],
    });
  }

  return {
    get isLoading() {
      return ipVariables.isLoading;
    },
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
});
