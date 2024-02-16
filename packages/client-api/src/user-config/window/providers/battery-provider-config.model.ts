import { z } from 'zod';

import { ProviderType } from '../provider-type.model';

export const BatteryProviderConfigSchema = z.object({
  type: z.literal(ProviderType.BATTERY),

  refresh_interval_ms: z.coerce.number().default(5 * 1000),
});

export type BatteryProviderConfig = z.infer<
  typeof BatteryProviderConfigSchema
>;
