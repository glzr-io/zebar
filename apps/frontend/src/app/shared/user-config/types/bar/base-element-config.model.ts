import { z } from 'zod';

import { createUniqueId } from '~/shared/utils';
import { ProviderConfigSchema } from './provider-config.model';

export const BaseElementConfigSchema = z.object({
  id: z.string().default(createUniqueId),
  class_name: z.string(),
  styles: z.string().optional(),
  providers: z
    // TODO
    .array(z.union([ProviderConfigSchema, z.enum(['cpu', 'gpu'])]))
    .default([]),
});

/** Base config for bar, groups, and components. */
export type BaseElementConfig = z.infer<typeof BaseElementConfigSchema>;
