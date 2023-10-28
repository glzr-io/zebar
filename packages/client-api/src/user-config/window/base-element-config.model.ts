import { z } from 'zod';

import { Prettify } from '~/utils';
import { ProvidersConfigSchema } from './providers-config.model';

export const BaseElementConfigSchema = z.object({
  id: z.string(),
  class_name: z.string(),
  styles: z.string().optional(),
  providers: ProvidersConfigSchema,
});

/** Base config for bar, groups, and components. */
export type BaseElementConfig = Prettify<
  z.infer<typeof BaseElementConfigSchema>
>;
