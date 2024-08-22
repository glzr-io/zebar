import type { Owner } from 'solid-js';

import {
  type IpVariables,
  createIpProvider,
} from '../ip/create-ip-provider';
import { WeatherStatus } from './weather-status.enum';
import { createProviderListener } from '../create-provider-listener';
import { ProviderType } from '../provider-type.model';

export interface WeatherProviderConfig {
  type: ProviderType.WEATHER;

  /**
   * Latitude to retrieve weather for. If not provided, latitude is instead
   * estimated based on public IP.
   */
  latitude?: number;

  /**
   * Longitude to retrieve weather for. If not provided, longitude is instead
   * estimated based on public IP.
   */
  longitude?: number;

  /**
   * How often this provider refreshes in milliseconds.
   */
  refresh_interval?: number;
}

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
          type: ProviderType.IP,
          refresh_interval: 60 * 60 * 1000,
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
