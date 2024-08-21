import { z } from 'zod';

import { ProviderType } from '../provider-type.model';

export const IpProviderConfigSchema = z.object({
  type: z.literal(ProviderType.IP),

  refresh_interval: z.coerce.number().default(60 * 60 * 1000),
});

export type IpProviderConfig = z.infer<typeof IpProviderConfigSchema>;
