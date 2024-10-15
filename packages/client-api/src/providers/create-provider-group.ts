import {
  createProvider,
  type ProviderConfig,
  type ProviderMap,
} from './create-provider';

/**
 * Config for creating multiple provider instances at once.
 *
 * Keys are unique identifiers for the provider instance, values are their
 * respective configs.
 */
export type ProviderGroupConfig = {
  [name: string]: ProviderConfig;
};

export type ProviderGroup<T extends ProviderGroupConfig> = {
  /**
   * A map of combined provider outputs. Each key corresponds to a provider
   * name, and each value is the output of that provider.
   */
  outputMap: {
    [TName in keyof T]: ProviderMap[T[TName]['type']]['output'];
  };

  /**
   * A map of combined provider errors. Each key corresponds to a provider
   * name, and each value is the error of that provider.
   */
  errorMap: {
    [TName in keyof T]: ProviderMap[T[TName]['type']]['error'];
  };

  /**
   * Whether the latest emission from any provider in the group is an
   * error.
   */
  hasErrors: boolean;

  /**
   * Underlying providers in the group.
   */
  raw: {
    [TName in keyof T]: ProviderMap[T[TName]['type']];
  };

  /**
   * Config for the provider group.
   */
  configMap: T;

  /**
   * Listens for outputs from any provider in the group.
   *
   * @param callback - Callback to run when an output is emitted.
   */
  onOutput: (
    callback: (outputMap: ProviderGroup<T>['outputMap']) => void,
  ) => void;

  /**
   * Listens for errors from any provider in the group.
   *
   * @param callback - Callback to run when an error is emitted.
   */
  onError: (
    callback: (errorMap: ProviderGroup<T>['errorMap']) => void,
  ) => void;

  /**
   * Restarts all providers in the group.
   */
  restartAll(): Promise<void>;

  /**
   * Stops all providers in the group.
   */
  stopAll(): Promise<void>;
};

/**
 * Creates multiple providers at once. A provider is a collection of
 * functions and variables that can change over time. Alternatively, a
 * single provider can be created using {@link createProvider}.
 */
export function createProviderGroup<T extends ProviderGroupConfig>(
  configMap: T,
): ProviderGroup<T> {
  const outputListeners = new Set<
    (outputMap: ProviderGroup<T>['outputMap']) => void
  >();

  const errorListeners = new Set<
    (errorMap: ProviderGroup<T>['errorMap']) => void
  >();

  const providerMap = createProviderMap(configMap);

  let outputMap = Object.fromEntries(
    Object.keys(providerMap).map(name => [name, null]),
  ) as ProviderGroup<T>['outputMap'];

  let errorMap = Object.fromEntries(
    Object.keys(providerMap).map(name => [name, null]),
  ) as ProviderGroup<T>['errorMap'];

  for (const [name, provider] of Object.entries(providerMap)) {
    provider.onOutput(() => {
      outputMap = { ...outputMap, [name]: provider.output };
      errorMap = { ...errorMap, [name]: null };
      outputListeners.forEach(listener => listener(outputMap));
    });

    provider.onError(() => {
      errorMap = { ...errorMap, [name]: provider.error };
      outputMap = { ...outputMap, [name]: null };
      errorListeners.forEach(listener => listener(errorMap));
    });
  }

  return {
    get outputMap() {
      return outputMap;
    },
    get errorMap() {
      return errorMap;
    },
    get hasErrors() {
      return Object.keys(errorMap).length > 0;
    },
    configMap,
    raw: providerMap,
    onOutput: callback => {
      outputListeners.add(callback);
    },
    onError: callback => {
      errorListeners.add(callback);
    },
    restartAll: async () => {
      await Promise.all(
        Object.values(providerMap).map(provider => provider.restart()),
      );
    },
    stopAll: async () => {
      outputListeners.clear();
      errorListeners.clear();

      await Promise.all(
        Object.values(providerMap).map(provider => provider.stop()),
      );
    },
  };
}

/**
 * Creates a map of names to provider instances.
 */
function createProviderMap<T extends ProviderGroupConfig>(configMap: T) {
  return Object.fromEntries(
    Object.entries(configMap).map(([name, config]) => [
      name,
      createProvider(config),
    ]),
  ) as ProviderGroup<T>['raw'];
}
