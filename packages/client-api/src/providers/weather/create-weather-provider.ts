import type { Owner } from 'solid-js';
import { z } from 'zod';

import {
  type IpProvider,
  createIpProvider,
} from '../ip/create-ip-provider';
import { WeatherStatus } from './weather-status.enum';
import { createProviderListener } from '../create-provider-listener';

export interface WeatherProviderConfig {
  type: 'weather';

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

const WeatherProviderConfigSchema = z.object({
  type: z.literal('weather'),
  latitude: z.coerce.number().optional(),
  longitude: z.coerce.number().optional(),
  refreshInterval: z.coerce.number().default(60 * 60 * 1000),
});

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
    ...WeatherProviderConfigSchema.parse(config),
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
          type: 'ip',
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
