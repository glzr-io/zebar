import { z } from 'zod';

import { TemplateError } from '~/template-engine';
import { createLogger } from '~/utils';
import { TemplatePropertyError } from './utils/template-property-error';

const logger = createLogger('parse-config-section');

export function parseConfigSection<
  T extends Record<string, unknown>,
  U extends z.AnyZodObject,
>(
  templateEngine: any,
  config: T,
  schema: U,
  contextData: Record<string, unknown>,
): z.infer<U> {
  const newConfigEntries = Object.entries(config).map(([key, value]) => {
    if (typeof value === 'string') {
      try {
        const rendered = templateEngine.render(value, contextData);
        return [key, rendered];
      } catch (err) {
        // Re-throw error as `TemplatePropertyError`.
        throw err instanceof TemplateError
          ? new TemplatePropertyError(
              err.message,
              key,
              value,
              err.templateIndex,
            )
          : err;
      }
    }

    return [key, value];
  });

  // TODO: Add logging for updated config here.
  const newConfig = Object.fromEntries(newConfigEntries);
  // console.log('Config updated:', newConfig);

  return schema.parse(newConfig);
}
