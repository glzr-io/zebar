import { Deferred } from '~/utils';
import type { ProviderConfig } from './create-provider';

export interface Provider<TConfig, TOutput> {
  /**
   * Latest output emitted from the provider.
   *
   * `null` if the latest emission from the provider is an error.
   */
  output: TOutput | null;

  /**
   * Latest error message emitted from the provider.
   *
   * `null` if the latest emission from the provider is a valid output.
   */
  error: string | null;

  /**
   * Whether the latest emission from the provider is an error.
   */
  hasError: boolean;

  /**
   * Underlying config for the provider.
   */
  config: TConfig;

  /**
   * Restarts the provider.
   */
  restart(): Promise<void>;

  /**
   * Stops the provider.
   */
  stop(): Promise<void>;

  /**
   * Listens for changes to the provider's value.
   *
   * @param callback - Callback to run when an output is emitted.
   */
  onOutput(callback: (output: TOutput) => void): void;

  /**
   * Listens for errors from the provider.
   *
   * @param callback - Callback to run when an error is emitted.
   */
  onError(callback: (error: string) => void): void;
}

type UnlistenFn = () => void | Promise<void>;

/**
 * Fetches next output or error from the provider.
 */
type ProviderFetcher<T> = (queue: {
  output: (nextOutput: T) => void;
  error: (nextError: string) => void;
}) => UnlistenFn | Promise<UnlistenFn>;

export async function createBaseProvider<
  TConfig extends ProviderConfig,
  TOutput,
>(
  config: TConfig,
  fetcher: ProviderFetcher<TOutput>,
): Promise<Provider<TConfig, TOutput>> {
  const outputListeners = new Set<(output: TOutput) => void>();
  const errorListeners = new Set<(error: string) => void>();

  let latestEmission = {
    output: null as TOutput | null,
    error: null as string | null,
    hasError: false,
  };

  let unlisten: UnlistenFn | null = await startFetcher();

  async function startFetcher() {
    const hasFirstEmit = new Deferred<void>();

    const unlisten = await fetcher({
      output: output => {
        latestEmission = { output, error: null, hasError: false };
        outputListeners.forEach(listener => listener(output));
        hasFirstEmit.resolve();
      },
      error: error => {
        latestEmission = { output: null, error, hasError: true };
        errorListeners.forEach(listener => listener(error));
        hasFirstEmit.resolve();
      },
    });

    // Wait for the first emission.
    await hasFirstEmit.promise;

    return unlisten;
  }

  return {
    get output() {
      return latestEmission.output;
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
    stop: async () => {
      outputListeners.clear();
      errorListeners.clear();

      if (unlisten) {
        await unlisten();
      }
    },
    onOutput: callback => {
      outputListeners.add(callback);
    },
    onError: callback => {
      errorListeners.add(callback);
    },
  };
}
