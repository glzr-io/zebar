import { Owner } from 'solid-js';

import { WeatherProviderConfig } from '~/user-config';
import { IpVariables, createIpProvider } from '../ip/create-ip-provider';
import { WeatherStatus } from './weather-status.enum';
import { createProviderListener } from '../create-provider-listener';

export interface WeatherVariables {
  isDaytime: boolean;
  status: WeatherStatus;
  celsiusTemp: number;
  fahrenheitTemp: number;
  windSpeed: number;
}

export async function createWeatherProvider(
  config: WeatherProviderConfig,
  owner: Owner,
) {
  let ipProvider: IpVariables | null = null;

  const mergedConfig = {
    ...config,
    longitude: config.longitude ?? (await getIpProvider()).approxLongitude,
    latitude: config.latitude ?? (await getIpProvider()).approxLatitude,
  };

  const weatherVariables = await createProviderListener<
    WeatherProviderConfig,
    WeatherVariables
  >(mergedConfig, owner);

  async function getIpProvider() {
    return (
      ipProvider ??
      (ipProvider = await createIpProvider(
        {
          type: 'ip',
          refresh_interval_ms: 60 * 60 * 1000,
        },
        owner,
      ))
    );
  }

  return {
    get isDaytime() {
      return weatherVariables().isDaytime;
    },
    get status() {
      return weatherVariables().status;
    },
    get celsiusTemp() {
      return weatherVariables().celsiusTemp;
    },
    get fahrenheitTemp() {
      return weatherVariables().fahrenheitTemp;
    },
    get windSpeed() {
      return weatherVariables().windSpeed;
    },
  };
}
