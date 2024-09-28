import { z } from 'zod';

import { createBaseProvider } from '../create-base-provider';
import { onProviderEmit } from '~/desktop';
import type {
  WeatherOutput,
  WeatherProvider,
  WeatherProviderConfig,
} from './weather-provider-types';

const weatherProviderConfigSchema = z.object({
  type: z.literal('weather'),
  latitude: z.coerce.number().optional(),
  longitude: z.coerce.number().optional(),
  refreshInterval: z.coerce.number().default(60 * 60 * 1000),
});

export function createWeatherProvider(
  config: WeatherProviderConfig,
): WeatherProvider {
  const mergedConfig = weatherProviderConfigSchema.parse(config);

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
