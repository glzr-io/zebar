import { z } from 'zod';

export const NetworkProviderOptionsSchema = z.object({
  refresh_interval_ms: z.coerce.number().default(5 * 1000),
});

export type NetworkProviderOptions = z.infer<
  typeof NetworkProviderOptionsSchema
>;

export const NetworkProviderConfigSchema = NetworkProviderOptionsSchema.extend({
  type: z.literal('network'),
});

export type NetworkProviderConfig = z.infer<typeof NetworkProviderConfigSchema>;
