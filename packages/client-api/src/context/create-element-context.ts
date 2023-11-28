import { Accessor, createComputed, createMemo } from 'solid-js';
import { createStore } from 'solid-js/store';

import {
  WindowConfigSchemaP1,
  BaseElementConfig,
  TemplateConfig,
  GroupConfig,
  ProvidersConfigSchema,
  parseConfigSection,
  TemplateConfigSchemaP1,
  GroupConfigSchemaP1,
  WindowConfig,
} from '~/user-config';
import { useTemplateEngine } from '~/template-engine';
import { createProvider } from './providers';
import { ElementContext, ElementType } from './shared';

export interface CreateElementContextArgs {
  id: string;
  config: WindowConfig | GroupConfig | TemplateConfig;
  ancestorVariables?: Accessor<Record<string, unknown>>[];
}

export function createElementContext(
  args: CreateElementContextArgs,
): ElementContext {
  const templateEngine = useTemplateEngine();
  const [elementContext, setElementContext] = createStore(getStoreValue());

  const elementVariables = createMemo(() => getElementVariables(args.config));

  const mergedVariables = createMemo(() => {
    const mergedAncestorVariables = (args.ancestorVariables ?? []).reduce(
      (acc, vars) => ({ ...acc, ...vars() }),
      {},
    );

    return {
      ...mergedAncestorVariables,
      ...elementVariables(),
    };
  });

  const type = getElementType();
  const childConfigs = getChildConfigs();
  const childIds = childConfigs.map(([_, value]) => value.id);

  createComputed(() => {
    const parsedConfig = parseConfigSection(
      templateEngine,
      { ...args.config, id: args.id },
      getSchemaForElement(type),
      mergedVariables(),
    );

    setElementContext({
      id: args.id,
      rawConfig: args.config,
      parsedConfig,
      data: mergedVariables(),
      type,
    });
  });

  /**
   * Get updated store value.
   */
  function getStoreValue() {
    const parsedConfig = parseConfigSection(
      templateEngine,
      { ...args.config, id: args.id },
      getSchemaForElement(type),
      mergedVariables(),
    );

    return {
      id: args.id,
      rawConfig: args.config,
      parsedConfig,
      data: mergedVariables(),
      type,
    };
  }

  function getElementType() {
    const [type] = args.id.split('/');

    // TODO: Validate in P1 schema instead.
    if (!['window', 'group', 'template'].includes(type)) {
      throw new Error(`Unrecognized element type '${type}'.`);
    }

    return type as ElementType;
  }

  // TODO: Validate in P1 schemas that `template/` and `group/` keys exist.
  function getSchemaForElement(type: ElementType) {
    switch (type) {
      case ElementType.WINDOW:
        return WindowConfigSchemaP1.strip();
      case ElementType.GROUP:
        return GroupConfigSchemaP1.strip();
      case ElementType.TEMPLATE:
        return TemplateConfigSchemaP1.strip();
    }
  }

  /**
   * Get child element configs.
   */
  function getChildConfigs() {
    return Object.entries(args.config).filter(
      (
        entry,
      ): entry is
        | [`group/${string}`, GroupConfig]
        | [`template/${string}`, TemplateConfig] => {
        const [key] = entry;
        return key.startsWith('group/') || key.startsWith('template/');
      },
    );
  }

  /**
   * Create element context for a child.
   */
  function createChildContext(id: string) {
    const foundConfig = childConfigs.find(([key]) => key === id);

    if (!foundConfig) {
      return null;
    }

    const [configKey, childConfig] = foundConfig;

    return createElementContext({
      config: childConfig,
      id: configKey,
      ancestorVariables: [...(args.ancestorVariables ?? []), elementVariables],
    });
  }

  function getElementVariables(config: BaseElementConfig) {
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
  }

  return {
    get id() {
      return elementContext.id;
    },
    get rawConfig() {
      return elementContext.rawConfig;
    },
    get parsedConfig() {
      return elementContext.parsedConfig;
    },
    get data() {
      return elementContext.data;
    },
    get type() {
      return elementContext.type;
    },
    childIds,
    getChild: createChildContext,
  };
}
