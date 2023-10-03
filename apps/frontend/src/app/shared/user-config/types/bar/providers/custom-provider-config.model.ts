import { z } from 'zod';

export const ScriptVariableConfigSchema = z.object({
  source: z.literal('script'),
  script_path: z.string(),
  script_args: z.string().optional(),
  refresh_interval_ms: z.coerce.number().default(5 * 1000),
});

export type ScriptVariableConfig = z.infer<typeof ScriptVariableConfigSchema>;

export const HttpVariableConfigSchema = z.object({
  source: z.literal('http'),
  url: z.string(),
  http_method: z.string().default('get'),
  refresh_interval_ms: z.coerce.number().default(5 * 1000),
});

export type HttpVariableConfig = z.infer<typeof HttpVariableConfigSchema>;

export const CustomProviderConfigSchema = z.object({
  type: z.literal('custom'),
  variables: z
    .record(
      z.string(),
      z.union([
        z.string(),
        z.boolean(),
        z.number(),
        ScriptVariableConfigSchema,
        HttpVariableConfigSchema,
      ]),
    )
    .optional(),
  commands: z.record(z.string(), z.string()).optional(),
});

export type CustomProviderConfig = z.infer<typeof CustomProviderConfigSchema>;
