import { z } from 'zod';

import { ProviderType } from '../provider-type.model';

export const NetworkActivityProviderConfigSchema = z.object({
  type: z.literal(ProviderType.NETWORKACTIVITY),

  refresh_interval: z.coerce.number().default(5 * 1000),
});

export type NetworkActivityProviderConfig = z.infer<typeof NetworkActivityProviderConfigSchema>;
