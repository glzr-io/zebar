import { z } from 'zod';

import { ComponentConfigSchema } from './component-config.model';

export const ComponentGroupConfigSchema = z.object({
  components: z.array(ComponentConfigSchema),
});

export type ComponentGroupConfig = z.infer<typeof ComponentGroupConfigSchema>;
