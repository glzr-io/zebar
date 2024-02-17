import { ZodError, type z } from 'zod';

/**
 * Parse a value with error formatting.
 */
export function parseWithSchema<T extends z.ZodType>(
  schema: T,
  value: unknown,
): z.infer<T> {
  try {
    return schema.parse(value);
  } catch (err) {
    if (err instanceof ZodError && err.errors.length) {
      const [firstError] = err.errors;
      const { message, path } = firstError!;
      const fullPath = path.join('.');

      throw new Error(
        `Property '${fullPath}' in config isn't valid.\n` +
          `⚠️ ${message}`,
      );
    }

    throw new Error('Failed to parse config.');
  }
}
