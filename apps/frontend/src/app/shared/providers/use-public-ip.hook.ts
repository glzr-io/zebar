import axios from 'axios';
import { createEffect, createResource, on } from 'solid-js';

import { memoize } from '../utils';
import { useLogger } from '../logging';

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
  const logger = useLogger('usePublicIp');

  const [publicIp] = createResource(() => {
    // Use https://ipinfo.io as provider for IP-related info.
    return axios
      .get<IpInfoApiResponse>('https://ipinfo.io/json')
      .then(({ data }) => ({
        ip: data.ip,
        city: data.city,
        country: data.country,
        latitude: data.loc.split(',')[0],
        longitude: data.loc.split(',')[1],
      }));
  });

  createEffect(
    on(publicIp, publicIp => logger.debug('Received IP data:', publicIp), {
      defer: true,
    }),
  );

  return publicIp;
});
