import { ZodError } from 'zod';

export function formatConfigError(err: unknown) {
  if (!(err instanceof Error)) {
    return new Error('Problem reading config file.');
  }

  if (err instanceof ZodError) {
    const [firstError] = err.errors;
    const { path, message } = firstError;
    const fullPath = path.join('.');

    return new Error(
      `Property '${fullPath}' in config isn't valid. Reason: '${message}'.`,
    );
  }

  return new Error(`Problem reading config file: ${(err as Error).message}.`);
}
