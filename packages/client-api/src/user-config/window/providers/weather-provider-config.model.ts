import { z } from 'zod';

import { ProviderType } from '../provider-type.model';

export const WeatherProviderConfigSchema = z.object({
  type: z.literal(ProviderType.WEATHER),

  /**
   * Latitude to retrieve weather for. If not provided, latitude is instead
   * estimated based on public IP.
   */
  latitude: z.coerce.number().optional(),

  /**
   * Longitude to retrieve weather for. If not provided, longitude is instead
   * estimated based on public IP.
   */
  longitude: z.coerce.number().optional(),

  /**
   * How often this component refreshes in milliseconds.
   */
  refresh_interval: z.coerce.number().default(60 * 60 * 1000),
});

export type WeatherProviderConfig = z.infer<
  typeof WeatherProviderConfigSchema
>;
