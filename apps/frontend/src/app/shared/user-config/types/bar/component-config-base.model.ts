import { z } from 'zod';

import { ScriptVariableConfigSchema } from '../script-variable-config.model';

export const ComponentConfigBaseSchema = z.object({
  id: z.string(),
  class_name: z.string(),
  style: z.string(),
  template_variables: z.record(
    z.string(),
    z.union([z.string(), ScriptVariableConfigSchema]),
  ),
  template_commands: z.record(z.string(), z.string()),
  template: z.string(),
  label: z.string(),
});

export type ComponentConfigBase = z.infer<typeof ComponentConfigBaseSchema>;
