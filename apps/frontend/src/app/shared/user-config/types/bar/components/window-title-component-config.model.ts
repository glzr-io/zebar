import { z } from 'zod';

import { ComponentConfigBaseSchema } from '../component-config-base.model';

export const WindowTitleComponentConfigSchema =
  ComponentConfigBaseSchema.extend({
    type: z.literal('window_title'),
    class_name: z.string().default('window-title-component'),
  });

export type WindowTitleComponentConfig = z.infer<
  typeof WindowTitleComponentConfigSchema
>;
