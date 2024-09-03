import { Deferred } from '~/utils';
import type { ProviderConfig } from './create-provider';

export interface Provider<TConfig, TVal> {
  /**
   * Current value of the provider.
   */
  value?: TVal;

  /**
   * Error message from the provider.
   */
  error?: string;

  /**
   * Whether the provider currently has an error.
   */
  hasError: boolean;

  /**
   * Config for the provider.
   */
  config: TConfig;

  /**
   * Restart the provider.
   */
  restart(): Promise<TVal>;

  /**
   * Stops the provider.
   */
  shutdown(): Promise<void>;

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
  const firstEmit = new Deferred<boolean>();

  const valueListeners: ((val: TVal) => void)[] = [];
  const errorListeners: ((error: string) => void)[] = [];

  let latestEmission = {
    value: null as TVal | null,
    error: null as string | null,
    hasError: false,
  };

  const shutdown = fetcher({
    value: value => {
      firstEmit.resolve(true);

      latestEmission = {
        value,
        error: null,
        hasError: false,
      };

      valueListeners.forEach(listener => listener(value));
    },
    error: error => {
      firstEmit.resolve(true);
      latestEmission = {
        value: null,
        error,
        hasError: false,
      };

      errorListeners.forEach(listener => listener(error));
    },
  });

  // Wait for the first emit before returning the provider.
  await firstEmit.promise;

  return {
    // @ts-ignore - TODO
    get value() {
      return latestEmission.value;
    },
    // @ts-ignore - TODO
    get error() {
      return latestEmission.error;
    },
    get hasError() {
      return latestEmission.hasError;
    },
    config,
    restart: async () => {
      // TODO: Implement restart.
      return null as any;
    },
    shutdown: async () => {
      valueListeners.length = 0;
      errorListeners.length = 0;
      const shutdownFn = await shutdown;
      await shutdownFn();
    },
    onValue: callback => {
      valueListeners.push(callback);
    },
    onError: callback => {
      errorListeners.push(callback);
    },
  };
}
