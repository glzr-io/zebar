export function simpleHash(...args: unknown[]): string {
  // JSON.stringify omits `undefined` and function values by default. These
  // need to be included in the hash.
  return JSON.stringify(args, (_: unknown, val: unknown) =>
    typeof val === 'object' ? val : String(val),
  );
}
