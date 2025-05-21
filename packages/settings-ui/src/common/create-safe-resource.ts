import {
  createResource as solidCreateResource,
  Resource,
  ResourceOptions,
  ResourceSource,
  ResourceFetcher,
  ResourceActions,
} from 'solid-js';

/**
 * Creates a resource that doesn't throw when accessed with an error state.
 *
 * This is a wrapper around SolidJS's `createResource` that safely returns
 * `undefined` instead of throwing when the resource is accessed in an error
 * state.
 *
 * @param source - The source signal for the resource
 * @param fetcher - The fetcher function that returns a promise
 * @param options - Resource options
 * @returns A tuple with a safe resource accessor and resource actions
 */
export function createSafeResource<T, S = undefined>(
  fetcher: ResourceFetcher<S, T>,
  options?: ResourceOptions<T>,
): [Resource<T>, ResourceActions<T>];

export function createSafeResource<T, S>(
  source: ResourceSource<S>,
  fetcher: ResourceFetcher<S, T>,
  options?: ResourceOptions<T>,
): [Resource<T>, ResourceActions<T>];

export function createSafeResource<T, S>(
  ...args: any[]
): [Resource<T>, ResourceActions<T>] {
  // Call the original `createResource` with the provided arguments.
  const [resource, actions] = (solidCreateResource as any)(...args);

  // Create a safe accessor function that doesn't throw on error.
  const accessor = function () {
    // If there's an error, return undefined instead of throwing.
    if (resource.error) {
      return undefined;
    }

    // Otherwise, return the resource value.
    return resource();
  };

  // Copy all properties from the original resource to our accessor.
  Object.defineProperties(
    accessor,
    Object.getOwnPropertyDescriptors(resource),
  );

  return [accessor as Resource<T>, actions];
}

// Export the original createResource as well for cases where you want the original behavior
export { solidCreateResource as createResource };
