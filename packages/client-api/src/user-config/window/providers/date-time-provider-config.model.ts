import { z } from 'zod';

export const DateTimeProviderConfigSchema = z.object({
  type: z.literal('date_time'),
  refresh_interval_ms: z.coerce.number().default(1000),
});

export type DateTimeProviderConfig = z.infer<
  typeof DateTimeProviderConfigSchema
>;
