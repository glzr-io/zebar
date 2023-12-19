import { z } from 'zod';

export const IpProviderConfigSchema = z.object({
  type: z.literal('ip'),
  refresh_interval_ms: z.coerce.number().default(60 * 60 * 1000),
});

export type IpProviderConfig = z.infer<typeof IpProviderConfigSchema>;
