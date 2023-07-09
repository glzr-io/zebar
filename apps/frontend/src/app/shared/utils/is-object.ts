/**
 * Whether value is an object literal.
 */
export function isObject(value: unknown): value is object {
  return value instanceof Object && !(value instanceof Array);
}
