import { z } from 'zod';

import { ProviderType } from '../provider-type.model';

export const LanguageProviderConfigSchema = z.object({
  type: z.literal(ProviderType.LANGUAGE),

  refresh_interval: z.coerce.number().default(5 * 1000),
});

export type LanguageProviderConfig = z.infer<
  typeof LanguageProviderConfigSchema
>;
