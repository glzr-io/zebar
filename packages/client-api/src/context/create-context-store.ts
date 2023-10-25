import { batch, createComputed, createMemo } from 'solid-js';
import { createStore } from 'solid-js/store';

import {
  BarConfigSchemaP1,
  BaseElementConfig,
  ComponentConfig,
  GroupConfig,
  ProvidersConfigSchema,
  UserConfig,
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

    createElementContext(windowId, windowConfig);
  }

  function createElementContext(
    id: string,
    config: BaseElementConfig,
    parentContext?: ElementContext,
  ) {
    const compoundId = parentContext
      ? `${parentContext.id}-${config.id}`
      : config.id;
    const path = getStorePath(config, parentContext) as any;

    const contextData = createMemo(() => ({
      ...rootVariables(),
      ...getElementVariables(config),
      // TODO: getAncestorVariables()
    }));

    createComputed(() => {
      const parsedConfig = parseConfigSection(
        { ...config, compoundId },
        BarConfigSchemaP1.strip(),
        contextData(),
      );

      const elementContext = {
        id: compoundId,
        parent: parentContext,
        children: [],
        rawConfig: config,
        parsedConfig,
        data: contextData,
      };

      // @ts-ignore
      setContextTree(...path, elementContext);

      const childConfigs = getChildConfigs(config);

      for (const [key, childConfig] of childConfigs) {
        const keyId = key.split('/')[1];
        const childId = parentContext ? `${parentContext.id}-${keyId}` : keyId;

        createElementContext(childId, childConfig, elementContext);
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
