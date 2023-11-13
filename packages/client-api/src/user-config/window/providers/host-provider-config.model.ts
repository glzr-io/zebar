import { z } from 'zod';

export const HostProviderConfigSchema = z.object({
  type: z.literal('host'),
  refresh_interval_ms: z.coerce.number().default(60 * 1000),
});

export type HostProviderConfig = z.infer<typeof HostProviderConfigSchema>;
