/**
 * Utility type for expanding a type to include all its properties.
 * Ref: https://youtube.com/shorts/2lCCKiWGlC0
 */
export type Prettify<T> = {
  [K in keyof T]: T[K];
} & {};
