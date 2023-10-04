type GetterProxyCallback = (target: object, key: string | symbol) => void;

export function createGetterProxy<T extends object>(
  target: T,
  callback: GetterProxyCallback,
): T {
  // Proxy cache is used to avoid creating a new proxy when a property is
  // accessed repeatedly.
  const proxyCache = new WeakMap();

  function wrap<U extends object>(target: U): U {
    if (proxyCache.has(target)) {
      return proxyCache.get(target);
    }

    const proxy = new Proxy(target, {
      get(target, key, receiver) {
        const value = Reflect.get(target, key, receiver);

        // Invoke callback with the object and the accessed key.
        callback(target, key);

        if (typeof value === 'object' && value !== null) {
          return wrap(value);
        }

        return value;
      },
    });

    proxyCache.set(proxy, target);
    return proxy;
  }

  return wrap(target);
}
