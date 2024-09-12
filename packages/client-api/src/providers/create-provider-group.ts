import {
  createProvider,
  type ProviderConfig,
  type ProviderMap,
} from './create-provider';

export type ProviderGroupConfig = {
  [name: string]: ProviderConfig;
};

export type ProviderGroupOutputs<T extends ProviderGroupConfig> = {
  [N in keyof T]: ProviderMap[T[N]['type']]['output'];
};

export type ProviderGroupErrors<T extends ProviderGroupConfig> = {
  [N in keyof T]: ProviderMap[T[N]['type']]['error'];
};

export type ProviderGroupRaw<T extends ProviderGroupConfig> = {
  [N in keyof T]: ProviderMap[T[N]['type']];
};

export type ProviderGroup<T extends ProviderGroupConfig> = {
  outputs: ProviderGroupOutputs<T>;

  errors: ProviderGroupErrors<T>;

  /**
   * Whether the group has any errors.
   */
  hasErrors: boolean;

  /**
   * Underlying providers for the group.
   */
  raw: ProviderGroupRaw<T>;

  /**
   * Config for the provider group.
   */
  config: T;

  /**
   * TODO
   * @param callback TODO
   */
  onOutput: (callback: (outputs: ProviderGroupOutputs<T>) => void) => void;

  /**
   * TODO
   * @param callback TODO
   */
  onError: (callback: (errors: ProviderGroupErrors<T>) => void) => void;

  /**
   * Restarts all providers in the group.
   */
  restartAll(): Promise<void>;

  /**
   * Stops all providers in the group.
   */
  stopAll(): Promise<void>;
};

const xxx = await createProviderGroup({
  glazewm: { type: 'glazewm' },
});

const config = xxx.config;
const output = xxx.outputs.glazewm;
const raw = xxx.raw.glazewm;

export async function createProviderGroup<T extends ProviderGroupConfig>(
  config: T,
): Promise<ProviderGroup<T>> {
  const outputListeners = new Set<
    (outputs: ProviderGroupOutputs<T>) => void
  >();

  const errorListeners = new Set<
    (errors: ProviderGroupErrors<T>) => void
  >();

  const providerEntries = await Promise.all([
    ...Object.entries(config).map(async ([key, value]) => {
      return [key, await createProvider(value)] as const;
    }),
  ]);

  const providers = Object.fromEntries(
    providerEntries,
  ) as ProviderGroupRaw<T>;

  let outputs = {} as ProviderGroupOutputs<T>;
  let errors = {} as ProviderGroupErrors<T>;

  for (const [name, provider] of providerEntries) {
    outputs = { ...outputs, [name]: provider.output };
    errors = { ...errors, [name]: provider.error };

    provider.onOutput(() => {
      outputs = { ...outputs, [name]: provider.output };
      outputListeners.forEach(listener => listener(outputs));
    });

    provider.onError(() => {
      errors = { ...errors, [name]: provider.error };
      errorListeners.forEach(listener => listener(errors));
    });
  }

  return {
    get outputs() {
      return outputs;
    },
    get errors() {
      return errors;
    },
    get hasErrors() {
      return Object.keys(errors).length > 0;
    },
    config,
    raw: providers,
    onOutput: callback => {
      outputListeners.add(callback);
    },
    onError: callback => {
      errorListeners.add(callback);
    },
    restartAll: async () => {
      await Promise.all(
        providerEntries.map(([_, provider]) => provider.restart()),
      );
    },
    stopAll: async () => {
      outputListeners.clear();
      errorListeners.clear();

      await Promise.all(
        providerEntries.map(([_, provider]) => provider.stop()),
      );
    },
  };
}
