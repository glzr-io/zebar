/**
 * Omits indexed keys from a TS interface.
 *
 * @example
 * ```typescript
 * interface MyInterface {
 *   static: string;
 *   [key: string]: unknown;
 * }
 * ExcludeIndexedKeys<MyInterface>; // { static: string; }
 * ```
 */
export type ExcludeIndexedKeys<T extends object> = {
  [K in keyof T as string extends K
    ? never
    : number extends K
    ? never
    : K]: T[K];
};
