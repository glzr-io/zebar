/**
 * Wraps a target object and deeply tracks property access.
 *
 * @param target Object to wrap.
 * @param callback Invoked on every property access with an array of keys to
 * the accessed value.
 */
export function createGetterProxy<T extends object>(
  target: T,
  callback: (path: (string | symbol)[]) => void,
): T {
  // Proxy cache is used to avoid creating a new proxy when a property is
  // accessed repeatedly.
  const proxyCache = new WeakMap();

  function wrap<U extends object>(
    target: U,
    parentPath: (string | symbol)[],
  ): U {
    if (proxyCache.has(target)) {
      return proxyCache.get(target);
    }

    const proxy = new Proxy(target, {
      get(target, key, receiver) {
        const value = Reflect.get(target, key, receiver);

        // Invoke callback with the path to the accessed key.
        const path = [...parentPath, key];
        callback(path);

        if (typeof value === 'object' && value !== null) {
          return wrap(value, path);
        }

        return value;
      },
    });

    proxyCache.set(proxy, target);
    return proxy;
  }

  return wrap(target, []);
}
