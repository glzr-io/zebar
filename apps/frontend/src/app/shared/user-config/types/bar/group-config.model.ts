import { z } from 'zod';

import { ComponentConfigSchema } from './component-config.model';
import { BaseElementConfigSchema } from './base-element-config.model';
import { Prettify } from '~/shared/utils';

export const GroupConfigSchema = BaseElementConfigSchema.extend({
  class_name: z.string().default('group'),
  components: z.array(ComponentConfigSchema).optional(),
});

export type GroupConfig = Prettify<z.infer<typeof GroupConfigSchema>>;
