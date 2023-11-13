import { simpleHash } from './simple-hash';

export type MemoizableFunction<
  A extends unknown[],
  R extends unknown,
  T extends unknown,
> = (this: T, ...args: A) => R;

/**
 * Utility for memoizing function calls.
 * Inspired by https://github.com/github/memoize.
 */
export function memoize<
  A extends unknown[],
  R extends unknown,
  T extends unknown,
>(fn: MemoizableFunction<A, R, T>): MemoizableFunction<A, R, T> {
  const cache = new Map();

  return function (this: T, ...args: A) {
    const id = simpleHash.apply(this, args);

    if (cache.has(id)) {
      return cache.get(id);
    }

    let result = fn.apply(this, args);

    if (result instanceof Promise) {
      result = result.catch(error => {
        cache.delete(id);
        throw error;
      }) as R;
    }

    cache.set(id, result);
    return result;
  };
}
