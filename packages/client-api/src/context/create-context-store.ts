import { batch, createComputed, createMemo } from 'solid-js';
import { createStore } from 'solid-js/store';

import {
  BarConfigSchemaP1,
  BaseElementConfig,
  ComponentConfig,
  GroupConfig,
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

  const rootVariables = createMemo(() => ({ env: configVariables()! }));

  createComputed(() => {
    batch(() => createContextTree());
  });

  function createContextTree() {
    const windowId = 'bar';
    const windowConfig = config[`bar/${windowId}`];

    createElementContext(windowConfig);
  }

  function createElementContext(
    config: BaseElementConfig,
    parentContext?: ElementContext,
  ) {
    const id = parentContext ? `${parentContext.id}-${config.id}` : config.id;
    const path = getStorePath(config, parentContext) as any;

    const contextData = createMemo(() => ({
      ...rootVariables(),
      ...getElementVariables(config),
      // TODO: getAncestorVariables()
    }));

    createComputed(() => {
      const parsedConfig = parseConfigSection(
        { ...config, id },
        BarConfigSchemaP1.strip(),
        contextData(),
      );

      const elementContext = {
        id,
        parent: parentContext,
        children: [],
        rawConfig: config,
        parsedConfig,
        data: contextData,
      };

      setContextTree(...path, elementContext);

      const childConfigs = getChildConfigs(config);

      for (const [_, childConfig] of childConfigs) {
        createElementContext(childConfig, elementContext);
      }
    });
  }

  function getStorePath(
    config: BaseElementConfig,
    parentContext?: ElementContext,
  ): string[] {
    const path = [config.id];
    let ancestorContext = parentContext;

    while (ancestorContext) {
      path.unshift('children');
      path.unshift(ancestorContext.id);
    }

    return path;
  }

  function getChildConfigs(config: BaseElementConfig) {
    return Object.entries(config).filter(
      ([key, value]) =>
        key.startsWith('component/') || key.startsWith('group/'),
      // TODO: Get rid of this type coercion.
    ) as any as [`component/${string}`, ComponentConfig | GroupConfig][];
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
