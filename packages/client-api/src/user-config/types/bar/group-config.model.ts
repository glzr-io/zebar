import { z } from 'zod';

import { ComponentConfigSchema } from './component-config.model';
import { BaseElementConfigSchema } from './base-element-config.model';
import { Prettify } from '../../../utils';

export const GroupConfigSchemaP1 = BaseElementConfigSchema.extend({
  class_name: z.string().default('group'),
});

export type GroupConfigP1 = Prettify<z.infer<typeof GroupConfigSchemaP1>>;

export const GroupConfigSchema = GroupConfigSchemaP1.extend({
  components: z.array(ComponentConfigSchema).default([]),
});

export type GroupConfig = Prettify<z.infer<typeof GroupConfigSchema>>;
