import { z } from 'zod';

import { ComponentConfigSchema } from './component-config.model';
import { ElementSchema } from '../shared/element.model';

export const ComponentGroupConfigSchema = ElementSchema.extend({
  components: z.array(ComponentConfigSchema).default([]),
});

export type ComponentGroupConfig = z.infer<typeof ComponentGroupConfigSchema>;
