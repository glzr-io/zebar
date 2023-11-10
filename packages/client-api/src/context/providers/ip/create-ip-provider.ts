import axios from 'axios';
import { onCleanup } from 'solid-js';
import { createStore } from 'solid-js/store';

import { memoize } from '~/utils';
import { IpProviderOptions, IpProviderOptionsSchema } from '~/user-config';
import { IpInfoApiResponse } from './ip-info-api-response.model';

const DEFAULT = IpProviderOptionsSchema.parse({});

export const createIpProvider = memoize(
  (options: IpProviderOptions = DEFAULT) => {
    const [ipVariables, setIpVariables] = createStore({
      is_loading: true,
      is_refreshing: true,
      address: '',
      approx_city: '',
      approx_country: '',
      approx_latitude: '',
      approx_longitude: '',
    });

    refresh();
    const interval = setInterval(() => refresh(), options.refresh_interval_ms);
    onCleanup(() => clearInterval(interval));

    async function refresh() {
      setIpVariables({ is_refreshing: true });

      // Use https://ipinfo.io as provider for IP-related info.
      const { data } = await axios.get<IpInfoApiResponse>(
        'https://ipinfo.io/json',
      );

      setIpVariables({
        is_loading: false,
        is_refreshing: false,
        address: data.ip,
        approx_city: data.city,
        approx_country: data.country,
        approx_latitude: data.loc.split(',')[0],
        approx_longitude: data.loc.split(',')[1],
      });
    }

    return {
      get is_loading() {
        return ipVariables.is_loading;
      },
      get is_refreshing() {
        return ipVariables.is_refreshing;
      },
      get address() {
        return ipVariables.address;
      },
      get approx_city() {
        return ipVariables.approx_city;
      },
      get approx_country() {
        return ipVariables.approx_country;
      },
      get approx_latitude() {
        return ipVariables.approx_latitude;
      },
      get approx_longitude() {
        return ipVariables.approx_longitude;
      },
      refresh,
    };
  },
);
