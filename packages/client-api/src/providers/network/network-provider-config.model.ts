import { z } from 'zod';

import { ProviderType } from '../provider-type.model';

export const NetworkProviderConfigSchema = z.object({
  type: z.literal(ProviderType.NETWORK),

  refresh_interval: z.coerce.number().default(5 * 1000),
});

export type NetworkProviderConfig = z.infer<
  typeof NetworkProviderConfigSchema
>;
