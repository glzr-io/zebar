import { createComputed, createMemo } from 'solid-js';
import { createStore } from 'solid-js/store';

import {
  BarConfigSchemaP1,
  BaseElementConfig,
  ProvidersConfigSchema,
  UserConfig,
  getBarConfigEntries,
} from '~/user-config';
import { createProvider } from './providers';
import { parseConfigSection } from '~/user-config/parse-config-section';
import { ElementContext } from './element-context.model';

export function createContextStore(
  config: UserConfig,
  configVariables: () => Record<string, unknown>,
) {
  const [contextTree, setContextTree] = createStore<ElementContext>(
    {} as ElementContext,
  );

  createComputed(createContextTree);

  function createContextTree() {
    const windowId = 'bar';
    const windowConfig = config[`bar/${windowId}`];

    const rootVariables = createMemo(() => ({
      env: configVariables()!,
    }));

    const windowVariables = createMemo(() => ({
      ...rootVariables(),
      ...getElementVariables(windowConfig),
    }));

    createComputed(() => {
      const parsedConfig = parseConfigSection(
        { ...windowConfig, id: windowId },
        BarConfigSchemaP1.strip(),
        windowVariables(),
      );

      setContextTree(windowId, parsedConfig);
    });
  }

  function createElementContext(
    config: BaseElementConfig,
    parentContext: ElementContext,
  ): ElementContext {
    const id = parentContext ? `${parentContext.id}-${config.id}` : config.id;

    return {
      id,
      parent: parentContext,
      children: '',
      rawConfig: '',
      parsedConfig: '',
      data: '',
    };
  }

  // TODO: Get variables from `variables` config as well.
  function getElementVariables(config: BaseElementConfig) {
    const providerConfigs = ProvidersConfigSchema.parse(
      config?.providers ?? [],
    );

    return providerConfigs.reduce(
      (acc, config) => ({
        ...acc,
        [config.type]: createProvider(config).variables,
      }),
      {},
    );
  }

  return {
    store: contextTree,
  };
}
