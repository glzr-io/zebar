/**
 * Whether value is not `null` or `undefined`.
 */
export function isDefined<T>(value: null | undefined | T): value is T {
  return value !== null && value !== undefined;
}
