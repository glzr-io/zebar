import { z } from 'zod';

export const ActiveWindowProviderConfigSchema = z.object({
  type: z.literal('active_window'),

  /** Substitution rules for formatting window title. */
  rewrite_title: z.record(z.string(), z.string()).default({}),
});

export type ActiveWindowProviderConfig = z.infer<
  typeof ActiveWindowProviderConfigSchema
>;
