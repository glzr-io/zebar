export type NonNullableArray<T extends readonly unknown[]> = {
  [K in keyof T]: NonNullable<T[K]>;
};
