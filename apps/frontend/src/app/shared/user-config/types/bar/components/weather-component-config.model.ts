import { z } from 'zod';

import { ElementSchema } from '../../shared/element.model';

export const WeatherComponentConfigSchema = ElementSchema.extend({
  type: z.literal('weather'),
  class_name: z.string().default('weather-component'),
  latitude: z.string().optional(),
  longitude: z.string().optional(),
});

export type WeatherComponentConfig = z.infer<
  typeof WeatherComponentConfigSchema
>;
