import { z } from 'zod';

import { ComponentConfig } from './component-config.model';

export const ComponentGroupConfig = z.object({
  components: z.array(ComponentConfig),
});

export type ComponentGroupConfig = z.infer<typeof ComponentGroupConfig>;
