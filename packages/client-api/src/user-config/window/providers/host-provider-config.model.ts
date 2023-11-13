import { z } from 'zod';

export const HostProviderOptionsSchema = z.object({
  refresh_interval_ms: z.coerce.number().default(5 * 1000),
});

export type HostProviderOptions = z.infer<typeof HostProviderOptionsSchema>;

export const HostProviderConfigSchema = HostProviderOptionsSchema.extend({
  type: z.literal('host'),
});

export type HostProviderConfig = z.infer<typeof HostProviderConfigSchema>;
