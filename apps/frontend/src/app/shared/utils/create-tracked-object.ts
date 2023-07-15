/**
 * @example
 * ```typescript
 * const obj = createTrackedObject<MyObject>();
 * console.log(obj.foo);
 * console.log(obj.getMemberAccesses()); // Output: ["foo"]
 * ```
 */
export function createTrackedObject<T extends object>() {
  const memberAccesses = new Set<string>();

  const handler: ProxyHandler<T> = {
    get(target, property, receiver) {
      // Track member access
      memberAccesses.add(property.toString());
      return Reflect.get(target, property, receiver);
    },
  };

  const trackedObject = new Proxy<T>({} as T, handler);

  // Method to get all member accesses
  // @ts-ignore - ugly
  trackedObject.getMemberAccesses = function () {
    return Array.from(memberAccesses);
  };

  return trackedObject;
}
