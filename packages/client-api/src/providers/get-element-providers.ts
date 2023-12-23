import {
  Accessor,
  Owner,
  createComputed,
  createSignal,
  runWithOwner,
} from 'solid-js';
import { createStore } from 'solid-js/store';

import { ProvidersConfigSchema } from '~/user-config';
import { ElementContext } from '~/element-context.model';
import { createProvider } from './create-provider';
import { PickPartial } from '~/utils';

export async function getElementProviders(
  elementContext: PickPartial<
    ElementContext,
    'parsedConfig' | 'providers'
  >,
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
      (elementContext.rawConfig as Record<string, unknown>)?.providers ??
        [],
    );

    // Create tuple of configs and the created provider.
    const providers = await Promise.all(
      providerConfigs.map(
        async config =>
          [
            config,
            await createProvider(elementContext, config, owner),
          ] as const,
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
