import { z } from 'zod';

export const ClockProviderConfigSchema = z.object({
  type: z.literal('clock'),
  refresh_interval_ms: z.number().default(1000),
});

export type ClockProviderConfig = z.infer<typeof ClockProviderConfigSchema>;
