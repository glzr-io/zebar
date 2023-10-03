import { z } from 'zod';

export const NetworkProviderConfigSchema = z.object({
  type: z.literal('network'),
  refresh_interval_ms: z.coerce.number().default(5 * 1000),
});

export type NetworkProviderConfig = z.infer<typeof NetworkProviderConfigSchema>;
