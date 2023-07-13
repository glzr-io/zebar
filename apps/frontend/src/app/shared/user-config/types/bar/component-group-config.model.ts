import { z } from 'zod';

import { ComponentConfigSchema } from './component-config.model';
import { ElementSchema } from '../shared/element.model';

export const ComponentGroupConfigSchema = ElementSchema.extend({
  class_name: z.string().default('group'),
  components: z.array(ComponentConfigSchema).default([]),
});

export type ComponentGroupConfig = z.infer<typeof ComponentGroupConfigSchema>;
