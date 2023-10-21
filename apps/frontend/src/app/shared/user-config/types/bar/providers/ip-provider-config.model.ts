import { z } from 'zod';

export const IpProviderOptionsSchema = z.object({
  refresh_interval_ms: z.coerce.number().default(60 * 1000),
});

export type IpProviderOptions = z.infer<typeof IpProviderOptionsSchema>;

export const IpProviderConfigSchema = IpProviderOptionsSchema.extend({
  type: z.literal('ip'),
});

export type IpProviderConfig = z.infer<typeof IpProviderConfigSchema>;
