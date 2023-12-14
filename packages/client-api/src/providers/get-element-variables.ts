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

export async function getElementVariables(
  config: WindowConfig | GroupConfig | TemplateConfig,
  ancestorVariables: Accessor<Record<string, unknown>>[],
  owner: Owner,
) {
  const [elementVariables, _] = createSignal(await getElementVariables());

  const [mergedVariables, setMergedVariables] =
    createStore(getMergedVariables());

  // Update the store on changes to any provider variables.
  runWithOwner(owner, () => {
    createComputed(() => setMergedVariables(getMergedVariables()));
  });

  /**
   * Get map of element providers.
   */
  async function getElementVariables() {
    const providerConfigs = ProvidersConfigSchema.parse(
      config?.providers ?? [],
    );

    // Create tuple of configs and the created provider.
    const providers = await Promise.all(
      providerConfigs.map(
        async config => [config, await createProvider(config, owner)] as const,
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
  function getMergedVariables() {
    const mergedAncestorVariables = (ancestorVariables ?? []).reduce(
      (acc, vars) => ({ ...acc, ...vars() }),
      {},
    );

    return {
      ...mergedAncestorVariables,
      ...elementVariables(),
    };
  }

  return {
    element: elementVariables,
    merged: mergedVariables,
  };
}
