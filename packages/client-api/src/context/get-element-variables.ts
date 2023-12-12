import { Accessor, createComputed, createMemo } from 'solid-js';
import { createStore } from 'solid-js/store';

import {
  TemplateConfig,
  GroupConfig,
  ProvidersConfigSchema,
  WindowConfig,
} from '~/user-config';
import { createProvider } from './providers';

export function getElementVariables(
  config: WindowConfig | GroupConfig | TemplateConfig,
  ancestorVariables?: Accessor<Record<string, unknown>>[],
) {
  const elementVariables = createMemo(() => {
    const providerConfigs = ProvidersConfigSchema.parse(
      config?.providers ?? [],
    );

    return providerConfigs.reduce(
      (acc, config) => ({
        ...acc,
        [config.type]: createProvider(config),
      }),
      {},
    );
  });

  const [mergedVariables, setMergedVariables] =
    createStore(getMergedVariables());

  // Update the store on changes to any provider variables.
  createComputed(() => setMergedVariables(getMergedVariables()));

  /**
   * Get updated store value.
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
