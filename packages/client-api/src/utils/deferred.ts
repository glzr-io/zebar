/**
 * Utility for creating a promise that can be resolved and rejected outside
 * the promise callback.
 *
 * @example
 * ```ts
 * const deferred = new Deferred<number>();
 * deferred.resolve(42);
 * ```
 */
export class Deferred<T> {
  promise: Promise<T>;
  resolve!: (val: T | PromiseLike<T>) => void;
  reject!: (err: any) => void;

  constructor() {
    this.promise = new Promise<T>((resolve, reject) => {
      this.resolve = resolve;
      this.reject = reject;
    });
  }
}
