import { NonNullableArray } from './types/non-nullable-array';
import { NonNullableObj } from './types/non-nullable-obj';
import { isDefined } from './is-defined';
import { isObject } from './is-object';

export function resolved<const T extends readonly unknown[]>(
  resources: T,
): NonNullableArray<T> | false;

export function resolved<const T extends Record<string, unknown>>(
  resourceMap: T,
): NonNullableObj<T> | false;

/**
 * Utility for returning either `false` or an array/object of non-nullable
 * values. Useful for waiting until all resources in `<Show when={...}>` have
 * been resolved.
 *
 * @example
 * ```typescript
 * resolved([myResource() as MyResource | undefined]); // `false | [MyResource]`
 * resolved(['' as string | null]); // `false | [string]`
 * resolved({ a: '' as string | null }); // `false | [{ a: string }]`
 * ```
 */
export function resolved(resourcesOrMap: unknown): unknown | false {
  if (!Array.isArray(resourcesOrMap) && !isObject(resourcesOrMap)) {
    throw new Error('Input is not an array or object.');
  }

  const isResolved = Array.isArray(resourcesOrMap)
    ? resourcesOrMap.every(isDefined)
    : Object.values(resourcesOrMap as Record<string, unknown>).every(isDefined);

  // If every resource is not undefined/null, then return array of resolved
  // resources.
  return isResolved ? resourcesOrMap : false;
}
