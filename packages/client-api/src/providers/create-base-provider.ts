import { Deferred } from '~/utils';
import type { ProviderConfig } from './create-provider';

export interface Provider<TConfig, TVal> {
  /**
   * Latest value emitted from the provider.
   *
   * `null` if the provider currently has an error.
   */
  value: TVal | null;

  /**
   * Latest error message emitted from the provider.
   *
   * `null` if the provider currently has a valid value.
   */
  error: string | null;

  /**
   * Whether the provider currently has an error.
   */
  hasError: boolean;

  /**
   * Config for the provider.
   */
  config: TConfig;

  /**
   * Restarts the provider.
   */
  restart(): Promise<void>;

  /**
   * Stops the provider.
   */
  destroy(): Promise<void>;

  /**
   * Listens for changes to the provider's value.
   * @param callback - Callback to run when the value changes.
   */
  onValue(callback: (nextVal: TVal) => void): void;

  /**
   * Listens for errors from the provider.
   * @param callback - Callback to run when an error is emitted.
   */
  onError(callback: (error: string) => void): void;
}

type UnlistenFn = () => void | Promise<void>;

/**
 * Fetches next value or error from the provider.
 */
type ProviderFetcher<T> = (queue: {
  value: (nextVal: T) => void;
  error: (nextError: string) => void;
}) => UnlistenFn | Promise<UnlistenFn>;

export async function createBaseProvider<
  TConfig extends ProviderConfig,
  TVal,
>(
  config: TConfig,
  fetcher: ProviderFetcher<TVal>,
): Promise<Provider<TConfig, TVal>> {
  const valueListeners = new Set<(val: TVal) => void>();
  const errorListeners = new Set<(error: string) => void>();

  let latestEmission = {
    value: null as TVal | null,
    error: null as string | null,
    hasError: false,
  };

  let unlisten: UnlistenFn | null = await startFetcher();

  async function startFetcher() {
    const hasFirstEmit = new Deferred<void>();

    const unlisten = await fetcher({
      value: value => {
        latestEmission = { value, error: null, hasError: false };
        valueListeners.forEach(listener => listener(value));
        hasFirstEmit.resolve();
      },
      error: error => {
        latestEmission = { value: null, error, hasError: true };
        errorListeners.forEach(listener => listener(error));
        hasFirstEmit.resolve();
      },
    });

    // Wait for the first emission.
    await hasFirstEmit.promise;

    return unlisten;
  }

  return {
    get value() {
      return latestEmission.value;
    },
    get error() {
      return latestEmission.error;
    },
    get hasError() {
      return latestEmission.hasError;
    },
    config,
    restart: async () => {
      if (unlisten) {
        await unlisten();
      }

      await startFetcher();
    },
    destroy: async () => {
      valueListeners.clear();
      errorListeners.clear();

      if (unlisten) {
        await unlisten();
      }
    },
    onValue: callback => {
      valueListeners.add(callback);
    },
    onError: callback => {
      errorListeners.add(callback);
    },
  };
}
