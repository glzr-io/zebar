import type { Owner } from 'solid-js';

import {
  type IpProvider,
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
  refreshInterval?: number;
}

export interface WeatherProvider {
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
  let ipProvider: IpProvider | null = null;

  const mergedConfig: WeatherProviderConfig = {
    ...config,
    refreshInterval: config.refreshInterval ?? 60 * 60 * 1000,
    longitude: config.longitude ?? (await getIpProvider()).approxLongitude,
    latitude: config.latitude ?? (await getIpProvider()).approxLatitude,
  };

  const weatherVariables = await createProviderListener<
    WeatherProviderConfig,
    WeatherProvider
  >(mergedConfig, owner);

  async function getIpProvider() {
    return (
      ipProvider ??
      (ipProvider = await createIpProvider(
        {
          type: ProviderType.IP,
          refreshInterval: 60 * 60 * 1000,
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
