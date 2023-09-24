import axios from 'axios';
import { createEffect, createResource, on } from 'solid-js';

import { memoize } from '../../utils';
import { useLogger } from '../../logging';
import { IpProviderConfig } from '../../user-config';
import { IpInfoApiResponse } from './ip-info-api-response.model';

export const useIpProvider = memoize((config: IpProviderConfig) => {
  const logger = useLogger('usePublicIp');

  const [ipData, { refetch }] = createResource(() => {
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
    on(ipData, ipData => logger.debug('Received IP data:', ipData), {
      defer: true,
    }),
  );

  return {
    // variables: ipData,
    variables: { x: 1 },
    commands: {
      refetch,
    },
  };
});
