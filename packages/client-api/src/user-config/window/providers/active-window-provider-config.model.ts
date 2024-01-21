import { z } from 'zod';

import { ProviderType } from '../provider-type.model';

export const ActiveWindowProviderConfigSchema = z.object({
  type: z.literal(ProviderType.ACTIVE_WINDOW),

  /** Substitution rules for formatting window title. */
  rewrite_title: z.record(z.string(), z.string()).default({}),
});

export type ActiveWindowProviderConfig = z.infer<
  typeof ActiveWindowProviderConfigSchema
>;
