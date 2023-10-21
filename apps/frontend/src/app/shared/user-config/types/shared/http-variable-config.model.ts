import { z } from 'zod';

export const HttpVariableConfigSchema = z.object({
  source: z.literal('http'),
  url: z.string(),
  http_method: z.string().default('get'),
  refresh_interval_ms: z.coerce.number().default(5 * 1000),
});

export type HttpVariableConfig = z.infer<typeof HttpVariableConfigSchema>;
