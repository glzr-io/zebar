import { z } from 'zod';

import { ScriptVariableConfigSchema } from '../shared/script-variable-config.model';
import { createUniqueId } from '~/shared/utils';

export const BaseElementConfigSchema = z.object({
  id: z.string().default(createUniqueId),
  class_name: z.string(),
  styles: z.string().optional(),
  template_variables: z
    .record(
      z.string(),
      z.union([
        z.string(),
        z.boolean(),
        z.number(),
        ScriptVariableConfigSchema,
      ]),
    )
    .optional(),
  template_commands: z.record(z.string(), z.string()).optional(),
  template: z.string().optional(),
  label: z.string().optional(),
});

/** Base config for bar, groups, and components. */
export type BaseElementConfig = z.infer<typeof BaseElementConfigSchema>;
