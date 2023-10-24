import { z } from 'zod';

export const WeatherProviderOptionsSchema = z.object({
  /**
   * Latitude to retrieve weather for. If not provided, latitude is instead
   * estimated based on public IP.
   */
  latitude: z.string().optional(),

  /**
   * Longitude to retrieve weather for. If not provided, longitude is instead
   * estimated based on public IP.
   */
  longitude: z.string().optional(),

  /* How often this component refreshes in milliseconds. */
  refresh_interval_ms: z.coerce.number().default(60 * 60 * 1000),
});

export type WeatherProviderOptions = z.infer<
  typeof WeatherProviderOptionsSchema
>;

export const WeatherProviderConfigSchema = WeatherProviderOptionsSchema.extend({
  type: z.literal('weather'),
});

export type WeatherProviderConfig = z.infer<typeof WeatherProviderConfigSchema>;
