import { z } from 'zod';

import { ScriptVariableConfigSchema } from './script-variable-config.model';
import { createUniqueId } from '~/shared/utils';

export const TemplateElementConfigSchema = z.object({
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

/** Base config for elements with a template. */
export type TemplateElementConfig = z.infer<typeof TemplateElementConfigSchema>;