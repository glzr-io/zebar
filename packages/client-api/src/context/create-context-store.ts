import {
  Accessor,
  Resource,
  createComputed,
  createEffect,
  createMemo,
} from 'solid-js';
import { createStore } from 'solid-js/store';

import { TemplateEngine } from '~/template-engine';
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
import { ElementContext } from './shared/element-context.model';
import { ElementType } from './shared/element-type.model';

// Context store path can actually be infinite depending on the number of child
// elements in the user's config.
type ContextStorePath =
  | []
  | ['children', number]
  | ['children', number, 'children', number];

interface CreateElementContextArgs {
  config: BaseElementConfig;
  configKey: string;
  path: ContextStorePath;
  ancestorData: Accessor<Record<string, unknown>>[];
}

export interface ContextStore {
  value: ElementContext | null;
  hasInitialized: boolean;
}

export function createContextStore(
  config: Resource<unknown>,
  configVariables: Resource<ConfigVariables>,
  templateEngine: TemplateEngine,
) {
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
      ancestorData: [rootVariables],
    });
  }

  function createElementContext(args: CreateElementContextArgs) {
    const { config, configKey, path, ancestorData } = args;

    const [typeString, id] = configKey.split('/');
    const type = getElementType(typeString);

    const elementData = createMemo(() => getElementData(config));

    const mergedData = createMemo(() => {
      const mergedAncestorData = ancestorData.reduce(
        (acc, data) => ({ ...acc, ...data() }),
        {},
      );

      return {
        ...mergedAncestorData,
        ...elementData(),
      };
    });

    createComputed(() => {
      const parsedConfig = parseConfigSection(
        templateEngine,
        { ...config, id },
        getSchemaForElement(type),
        mergedData(),
      );

      // @ts-ignore - TODO
      setContextTree('value', ...path, {
        id,
        children: [],
        rawConfig: config,
        parsedConfig,
        data: mergedData(),
        type,
        getChild: function (id: string) {
          const children = getChildConfigs(config);

          const [configKey, childConfig] = children.find(
            ([key]) => key === id,
          )!;

          const index = children.findIndex(([key]) => key === id)!;

          const childContext = getChildContext({
            config: childConfig,
            configKey,
            path: [...path, 'children', index] as ContextStorePath,
            ancestorData: [...ancestorData, elementData],
          });
          console.log(
            'childContext',
            childContext,
            children,
            configKey,
            childConfig,
            index,
          );
          return childContext;
        },
      });
    });

    // for (const [index, entry] of getChildConfigs(config).entries()) {
    //   const [configKey, childConfig] = entry;

    //   createElementContext({
    //     config: childConfig,
    //     configKey,
    //     path: [...path, 'children', index] as ContextStorePath,
    //     ancestorData: [...ancestorData, elementData],
    //   });
    // }
  }

  function getChildContext(args: CreateElementContextArgs) {
    const { config, configKey, path, ancestorData } = args;

    const [typeString, id] = configKey.split('/');
    const type = getElementType(typeString);

    const elementData = createMemo(() => getElementData(config));

    const mergedData = createMemo(() => {
      const mergedAncestorData = ancestorData.reduce(
        (acc, data) => ({ ...acc, ...data() }),
        {},
      );

      return {
        ...mergedAncestorData,
        ...elementData(),
      };
    });

    const [childContext, setChildContext] = createStore<ElementContext>(
      {} as ElementContext,
    );

    createComputed(() => {
      const parsedConfig = parseConfigSection(
        templateEngine,
        { ...config, id },
        getSchemaForElement(type),
        mergedData(),
      );

      setChildContext({
        id,
        children: [],
        rawConfig: config,
        parsedConfig,
        data: mergedData(),
        type,
        getChild: function (id: string) {
          const children = getChildConfigs(config);

          console.log('children1', children, id);

          const [configKey, childConfig] = children.find(
            ([key]) => key === id,
          )!;

          console.log('children2', children);
          const index = children.findIndex(([key]) => key === id)!;

          const childContext = getChildContext({
            config: childConfig,
            configKey,
            path: [...path, 'children', index] as ContextStorePath,
            ancestorData: [...ancestorData, elementData],
          });
          console.log(
            'childContext',
            childContext,
            children,
            configKey,
            childConfig,
            index,
          );
          return childContext;
        },
      });
    });

    return childContext;
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
      ([key]) => key.startsWith('template/') || key.startsWith('group/'),
      // TODO: Get rid of this type coercion.
    ) as any as [
      `template/${string}` | `group/${string}`,
      TemplateConfig | GroupConfig,
    ][];
  }

  // TODO: Get variables from `variables` config as well.
  function getElementData(config: BaseElementConfig) {
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
    store: contextTree,
  };
}
