import { z } from 'zod';

export const DateTimeProviderOptionsSchema = z.object({
  refresh_interval_ms: z.coerce.number().default(1000),
});

export type DateTimeProviderOptions = z.infer<
  typeof DateTimeProviderOptionsSchema
>;

export const DateTimeProviderConfigSchema =
  DateTimeProviderOptionsSchema.extend({
    type: z.literal('date_time'),
  });

export type DateTimeProviderConfig = z.infer<
  typeof DateTimeProviderConfigSchema
>;
