import { z } from 'zod';

export const IpProviderConfigSchema = z.object({
  type: z.literal('ip'),
  refresh_interval_ms: z.number().default(60 * 1000),
});

export type IpProviderConfig = z.infer<typeof IpProviderConfigSchema>;
