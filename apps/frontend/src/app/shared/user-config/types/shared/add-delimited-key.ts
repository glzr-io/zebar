import { RefinementCtx, ZodType, z } from 'zod';

/**
 * Adds a key delimited by `/` to an object schema.
 *
 * @example
 * ```typescript
 * z.object({}).superRefine(addDelimitedKey('test', TestSchema)) // { [key: `test/${string}`]: Test }
 * ```
 */
export function addDelimitedKey<const T extends string, U extends ZodType>(
  keyPrefix: T,
  valueSchema: U,
) {
  return <V extends { [key: string]: unknown }>(
    arg: V,
    ctx: RefinementCtx,
  ): arg is V & {
    [key in `${T}/${string}`]: z.output<U>;
  } => {
    // Find keys that have the given prefix.
    const delimitedKeys = Object.keys(arg).filter(key =>
      key.startsWith(keyPrefix),
    );

    for (const key of delimitedKeys) {
      const res = valueSchema.safeParse(arg[key]);

      if (res.success) {
        arg[key as keyof V] = res.data;
        continue;
      }

      for (const issue of res.error.issues) {
        ctx.addIssue({
          ...issue,
          path: [key, ...issue.path],
        });
      }
    }

    return z.NEVER;
  };
}
