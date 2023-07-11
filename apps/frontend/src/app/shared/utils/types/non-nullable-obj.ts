export type NonNullableObj<T extends Record<string, unknown>> = {
  [K in keyof T]: NonNullable<T[K]>;
};
