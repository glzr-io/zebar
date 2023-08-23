import { z } from 'zod';

import { ComponentConfigBaseSchema } from '../component-config-base.model';
import { withSlotSchema } from '../with-slot-schema';

export const WeatherComponentConfigSchema = withSlotSchema(
  ComponentConfigBaseSchema.extend({
    type: z.literal('weather'),
    class_name: z.string().default('weather-component'),
    slot: z.string().default('{{ celsius_temp }}°C'),
    // Latitude to retrieve weather for. If not provided, latitude is instead
    // estimated based on public IP.
    latitude: z.string().optional(),
    // Longitude to retrieve weather for. If not provided, longitude is instead
    // estimated based on public IP.
    longitude: z.string().optional(),
    // How often this component refreshes in milliseconds.
    refresh_interval_ms: z.number().default(60 * 60 * 1000),
  }),
);

export type WeatherComponentConfig = z.infer<
  typeof WeatherComponentConfigSchema
>;
