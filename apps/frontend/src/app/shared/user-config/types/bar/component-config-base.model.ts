import { z } from 'zod';

import { ScriptVariableConfig } from '../script-variable-config.model';

export const ComponentConfigBase = z.object({
  id: z.string(),
  class_name: z.string(),
  style: z.string(),
  template_variables: z.record(
    z.string(),
    z.union([z.string(), ScriptVariableConfig]),
  ),
  template_commands: z.record(z.string(), z.string()),
  template: z.string(),
  label: z.string(),
});

export type ComponentConfigBase = z.infer<typeof ComponentConfigBase>;
