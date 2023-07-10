import { z } from 'zod';

import { ComponentConfigBase } from '../component-config-base.model';

export const ClockComponentConfig = ComponentConfigBase.extend({
  type: z.literal('clock'),
});

export type ClockComponentConfig = z.infer<typeof ClockComponentConfig>;
