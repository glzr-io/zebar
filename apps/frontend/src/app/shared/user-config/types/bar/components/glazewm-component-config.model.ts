import { z } from 'zod';

import { ComponentConfigBase } from '../component-config-base.model';

export const GlazeWMComponentConfig = ComponentConfigBase.extend({
  type: z.literal('glazewm'),
});

export type GlazeWMComponentConfig = z.infer<typeof GlazeWMComponentConfig>;
