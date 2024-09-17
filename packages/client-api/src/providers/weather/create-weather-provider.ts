import { z } from 'zod';

import type { IpProvider } from '../ip/create-ip-provider';
import { WeatherStatus } from './weather-status.enum';
import {
  createBaseProvider,
  type Provider,
} from '../create-base-provider';
import { onProviderEmit } from '~/desktop';
import { createProvider } from '../create-provider';

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

const weatherProviderConfigSchema = z.object({
  type: z.literal('weather'),
  latitude: z.coerce.number().optional(),
  longitude: z.coerce.number().optional(),
  refreshInterval: z.coerce.number().default(60 * 60 * 1000),
});

export type WeatherProvider = Provider<
  WeatherProviderConfig,
  WeatherOutput
>;

export interface WeatherOutput {
  isDaytime: boolean;
  status: WeatherStatus;
  celsiusTemp: number;
  fahrenheitTemp: number;
  windSpeed: number;
}

export async function createWeatherProvider(
  config: WeatherProviderConfig,
): Promise<WeatherProvider> {
  let ipProvider: IpProvider | null = null;

  const mergedConfig: WeatherProviderConfig = {
    ...weatherProviderConfigSchema.parse(config),
    longitude:
      config.longitude ?? (await getIpProvider()).output!.approxLongitude,
    latitude:
      config.latitude ?? (await getIpProvider()).output!.approxLatitude,
  };

  async function getIpProvider() {
    return (
      ipProvider ?? (ipProvider = await createProvider({ type: 'ip' }))
    );
  }

  return createBaseProvider(mergedConfig, async queue => {
    return onProviderEmit<WeatherOutput>(mergedConfig, ({ result }) => {
      if ('error' in result) {
        queue.error(result.error);
      } else {
        queue.output(result.output);
      }
    });
  });
}
