import {
  Accessor,
  ResourceReturn,
  createComputed,
  createEffect,
  createMemo,
} from 'solid-js';
import { createStore } from 'solid-js/store';

import { createTemplateEngine } from '~/template-engine';
import {
  WindowConfigSchemaP1,
  BaseElementConfig,
  TemplateConfig,
  GroupConfig,
  ProvidersConfigSchema,
  parseConfigSection,
  TemplateConfigSchemaP1,
  GroupConfigSchemaP1,
  UserConfig,
  formatConfigError,
  ConfigVariables,
} from '~/user-config';
import { createProvider } from './providers';
import { ElementContext } from './element-context.model';
import { ElementType } from './element-type.model';

type ContextStorePath =
  | []
  | ['children', number]
  | ['children', number, 'children', number];

interface CreateElementContextArgs {
  config: BaseElementConfig;
  configKey: string;
  path: ContextStorePath;
  parentPath?: ContextStorePath;
  ancestorContexts: Accessor<Record<string, unknown>>[];
}

export interface ContextStore {
  value: ElementContext | null;
  hasInitialized: boolean;
}

export function createContextStore(
  configResource: ResourceReturn<unknown, unknown>,
  configVariablesResource: ResourceReturn<ConfigVariables, unknown>,
) {
  const [config, { refetch: reloadConfig }] = configResource;
  const [configVariables] = configVariablesResource;

  const templateEngine = createTemplateEngine();

  const [contextTree, setContextTree] = createStore<ContextStore>({
    value: null,
    hasInitialized: false,
  });

  const rootVariables = createMemo(() => ({ env: configVariables() }));

  // Initialize context tree when config and config variables are ready.
  createEffect(() => {
    if (config() && configVariables()) {
      try {
        createContextTree();
        setContextTree({ hasInitialized: true });
      } catch (err) {
        throw formatConfigError(err);
      }
    }
  });

  function createContextTree() {
    // TODO: Get window to open from launch args.
    const configKey = 'window/bar';
    const windowConfig = (config() as UserConfig)[configKey];

    createElementContext({
      config: windowConfig,
      configKey,
      path: [],
      ancestorContexts: [rootVariables],
    });
  }

  function createElementContext(args: CreateElementContextArgs) {
    const { config, configKey, path, parentPath, ancestorContexts } = args;

    const [typeString, id] = configKey.split('/');
    const type = getElementType(typeString);

    const elementContext = createMemo(() => getElementVariables(config));

    const mergedContext = createMemo(() => {
      const ancestorContext = ancestorContexts.reduce(
        (acc, context) => ({ ...acc, ...context() }),
        {},
      );

      const mergedContext = {
        ...ancestorContext,
        ...elementContext(),
      };

      // TODO: Removing this console log removes reactivity.
      console.log('Context updated', mergedContext);
      return mergedContext;
    });

    createComputed(() => {
      const parsedConfig = parseConfigSection(
        templateEngine,
        { ...config, id },
        getSchemaForElement(type),
        mergedContext(),
      );

      // @ts-ignore - TODO
      setContextTree('value', ...path, {
        id,
        children: [],
        rawConfig: config,
        parsedConfig,
        data: mergedContext(),
        type,
      });
    });

    for (const [index, entry] of getChildConfigs(config).entries()) {
      const [configKey, childConfig] = entry;

      createElementContext({
        config: childConfig,
        configKey,
        path: [...path, 'children', index] as ContextStorePath,
        parentPath: path,
        ancestorContexts: [...ancestorContexts, elementContext],
      });
    }
  }

  function getElementType(type: string) {
    switch (type) {
      case 'window':
        return ElementType.WINDOW;
      case 'group':
        return ElementType.GROUP;
      case 'template':
        return ElementType.TEMPLATE;
      default:
        throw new Error(`Unrecognized element type '${type}'.`);
    }
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

  function getChildConfigs(config: BaseElementConfig) {
    return Object.entries(config).filter(
      ([key, value]) => key.startsWith('template/') || key.startsWith('group/'),
      // TODO: Get rid of this type coercion.
    ) as any as [
      `template/${string}` | `group/${string}`,
      TemplateConfig | GroupConfig,
    ][];
  }

  // TODO: Get variables from `variables` config as well.
  function getElementVariables(config: BaseElementConfig) {
    const providerConfigs = ProvidersConfigSchema.parse(
      config?.providers ?? [],
    );

    return providerConfigs.reduce(
      (acc, config) => ({
        ...acc,
        // TODO: Remove `variables` and `commands` properties on providers.
        [config.type]: createProvider(config).variables,
      }),
      {},
    );
  }

  async function reload() {
    reloadConfig();
  }

  return {
    store: contextTree,
    reload,
  };
}
