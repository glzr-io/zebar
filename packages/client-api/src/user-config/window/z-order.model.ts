import { z } from 'zod';

export const ZOrderSchema = z
  .enum(['always_on_top', 'always_on_bottom', 'normal'])
  .default('normal');

export type ZOrder = z.infer<typeof ZOrderSchema>;
