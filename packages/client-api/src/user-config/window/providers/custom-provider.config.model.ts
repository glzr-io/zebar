import { z } from 'zod';

import { ProviderType } from '../provider-type.model';
import { FnPathSchema } from '~/user-config/shared';

export const CustomProviderConfigSchema = z.union([
  z.object({
    type: z.literal(ProviderType.CUSTOM),
    refresh_interval: z.coerce.number().default(5 * 1000),
    start_fn_path: FnPathSchema.optional(),
    refresh_fn_path: FnPathSchema,
    stop_fn_path: FnPathSchema.optional(),
  }),
  z.object({
    type: z.literal(ProviderType.CUSTOM),
    start_fn_path: FnPathSchema.optional(),
    emitter_fn_path: FnPathSchema,
    stop_fn_path: FnPathSchema.optional(),
  }),
]);

export type CustomProviderConfig = z.infer<
  typeof CustomProviderConfigSchema
>;
