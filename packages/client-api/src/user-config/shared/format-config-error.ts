import { ZodError } from 'zod';

import { TemplatePropertyError } from './template-property-error';

export function formatConfigError(err: unknown) {
  if (!(err instanceof Error)) {
    return new Error('Problem reading config file.');
  }

  if (err instanceof ZodError && err.errors.length) {
    const [firstError] = err.errors;
    const { message, path } = firstError!;
    const fullPath = path.join('.');

    return new Error(
      `Property '${fullPath}' in config isn't valid.\n` + `⚠️ ${message}`,
    );
  }

  if (err instanceof TemplatePropertyError) {
    const { message, path, template, templateIndex } = err;

    return new Error(
      `Property '${path}' in config isn't valid.\n\n` +
        'Syntax error at:\n' +
        `...${template.slice(templateIndex - 30, templateIndex)} << \n\n` +
        `⚠️ ${message}`,
    );
  }

  return new Error(
    `Problem reading config file: ${(err as Error).message}.`,
  );
}
