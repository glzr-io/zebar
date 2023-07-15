import axios from 'axios';
import { createResource } from 'solid-js';

import { memoize } from '../utils';

export interface IpInfoApiResponse {
  ip: string;
  city: string;
  region: string;
  country: string;
  loc: string;
  org: string;
  postal: string;
  timezone: string;
  readme: string;
}

export const usePublicIp = memoize(() => {
  const [publicIp] = createResource(async () => {
    // Use https://ipinfo.io as provider for IP-related info.
    const publicIp = await axios
      .get<string>('https://ipinfo.io/ip')
      .then(({ data }) => data);

    return axios
      .get<IpInfoApiResponse>(`https://ipinfo.io/${publicIp}/json`)
      .then(({ data }) => ({
        ip: data.ip,
        city: data.city,
        country: data.country,
        latitude: data.loc.split(',')[0],
        longitude: data.loc.split(',')[1],
      }));
  });

  return publicIp;
});
