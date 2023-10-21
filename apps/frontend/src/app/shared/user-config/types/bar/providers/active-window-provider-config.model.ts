import { z } from 'zod';

export const ActiveWindowProviderOptionsSchema = z.object({
  /** Substitution rules for formatting window title. */
  rewrite_title: z.record(z.string(), z.string()),
});

export type ActiveWindowProviderOptions = z.infer<
  typeof ActiveWindowProviderOptionsSchema
>;

export const ActiveWindowProviderConfigSchema =
  ActiveWindowProviderOptionsSchema.extend({
    type: z.literal('active_window'),
  });

export type ActiveWindowProviderConfig = z.infer<
  typeof ActiveWindowProviderConfigSchema
>;
