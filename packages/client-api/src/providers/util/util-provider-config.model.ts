import { z } from 'zod';

import { ProviderType } from '../provider-type.model';

export const UtilProviderConfigSchema = z.object({
  type: z.literal(ProviderType.UTIL),
});

export type UtilProviderConfig = z.infer<typeof UtilProviderConfigSchema>;
