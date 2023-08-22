import { z } from 'zod';

export type WithDynamicKeyOptions<
  TKey extends string,
  TSub extends z.ZodType,
> = {
  isKey: (key: string) => key is TKey;
  schema: TSub;
};

/**
 * Adds a dynamic key to an object schema.
 *
 * @example
 * ```typescript
 * const MySchema = withDynamicKey(z.object({}), {
 *   isKey: (key: string): key is `sub/${string}` => key.startsWith('sub/'),
 *   schema: SubSchema,
 * }); // {} & { x: `sub/${string}`]: SubSchema }
 * ```
 */
export function withDynamicKey<
  TObject extends z.AnyZodObject,
  const TKey extends string,
  TSub extends z.ZodType,
>(schema: TObject, options: WithDynamicKeyOptions<TKey, TSub>) {
  // `passthrough` is needed here to allow the dynamic key. The resulting type
  // is then narrowed down and validated within `transform`.
  return schema.passthrough().transform((val, ctx) => {
    const defaultKeys = Object.keys(schema.shape);

    for (const key of Object.keys(val)) {
      if (defaultKeys.includes(key)) {
        continue;
      }

      if (!options.isKey(key)) {
        ctx.addIssue({
          code: z.ZodIssueCode.unrecognized_keys,
          keys: [key],
          path: [key],
        });
      }
    }

    const dynamicKeys = Object.keys(val).filter(key => options.isKey(key));

    for (const dynamicKey of dynamicKeys) {
      const res = options.schema.safeParse(val[dynamicKey]);

      if (res.success) {
        val[dynamicKey] = res.data;
        continue;
      }

      for (const issue of res.error.issues) {
        ctx.addIssue({
          ...issue,
          path: [dynamicKey, ...issue.path],
        });
      }
    }

    return val as z.infer<TObject> & {
      [key in TKey]: z.infer<TSub>;
    };
  });
}
