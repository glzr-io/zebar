import { z } from 'zod';

import { ScriptVariableConfigSchema } from './script-variable-config.model';
import { createUniqueId } from '~/shared/utils';

export const ElementSchema = z
  .object({
    id: z.string().default(createUniqueId),
    class_name: z.string(),
    styles: z.string(),
    template_variables: z.record(
      z.string(),
      z.union([z.string(), ScriptVariableConfigSchema]),
    ),
    template_commands: z.record(z.string(), z.string()),
    template: z.string(),
    label: z.string(),
  })
  .partial()
  .required({ id: true });

/** Base type for elements with a template. */
export type Element = z.infer<typeof ElementSchema>;
