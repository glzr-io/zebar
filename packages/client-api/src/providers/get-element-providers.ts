import {
  Accessor,
  Owner,
  createComputed,
  createSignal,
  runWithOwner,
} from 'solid-js';
import { createStore } from 'solid-js/store';

import {
  TemplateConfig,
  GroupConfig,
  ProvidersConfigSchema,
  WindowConfig,
} from '~/user-config';
import { createProvider } from '.';

export async function getElementProviders(
  config: WindowConfig | GroupConfig | TemplateConfig,
  ancestorProviders: Accessor<Record<string, unknown>>[],
  owner: Owner,
) {
  const [elementProviders, _] = createSignal(await getElementProviders());

  const [mergedProviders, setMergedProviders] = createStore(
    getMergedProviders(),
  );

  // Update the store on changes to any provider variables.
  runWithOwner(owner, () => {
    createComputed(() => setMergedProviders(getMergedProviders()));
  });

  /**
   * Get map of element providers.
   */
  async function getElementProviders() {
    const providerConfigs = ProvidersConfigSchema.parse(
      config?.providers ?? [],
    );

    // Create tuple of configs and the created provider.
    const providers = await Promise.all(
      providerConfigs.map(
        async config =>
          [config, await createProvider(config, owner)] as const,
      ),
    );

    return providers.reduce(
      (acc, [config, provider]) => ({
        ...acc,
        [config.type]: provider,
      }),
      {},
    );
  }

  /**
   * Get map of element providers merged with ancestor providers.
   */
  function getMergedProviders() {
    const mergedancestorProviders = (ancestorProviders ?? []).reduce(
      (acc, vars) => ({ ...acc, ...vars() }),
      {},
    );

    return {
      ...mergedancestorProviders,
      ...elementProviders(),
    };
  }

  return {
    element: elementProviders,
    merged: mergedProviders,
  };
}
