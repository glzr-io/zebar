import { z } from 'zod';

export const BatteryProviderOptionsSchema = z.object({
  refresh_interval_ms: z.coerce.number().default(60 * 1000),
});

export type BatteryProviderOptions = z.infer<
  typeof BatteryProviderOptionsSchema
>;

export const BatteryProviderConfigSchema = BatteryProviderOptionsSchema.extend({
  type: z.literal('battery'),
});

export type BatteryProviderConfig = z.infer<typeof BatteryProviderConfigSchema>;
