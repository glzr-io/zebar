import { z } from 'zod';

export const GlazewmProviderOptionsSchema = z.object({});

export type GlazewmProviderOptions = z.infer<
  typeof GlazewmProviderOptionsSchema
>;

export const GlazewmProviderConfigSchema = GlazewmProviderOptionsSchema.extend({
  type: z.literal('glazewm'),
});

export type GlazewmProviderConfig = z.infer<typeof GlazewmProviderConfigSchema>;
