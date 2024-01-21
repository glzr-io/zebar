import { z } from 'zod';

import { ProviderType } from '../provider-type.model';

export const MonitorsProviderConfigSchema = z.object({
  type: z.literal(ProviderType.MONITORS),
});

export type MonitorsProviderConfig = z.infer<
  typeof MonitorsProviderConfigSchema
>;
