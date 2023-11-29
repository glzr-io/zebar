import { Accessor, createEffect, createResource } from 'solid-js';

import {
  GlobalConfigSchema,
  UserConfig,
  WindowConfig,
  buildStyles,
  getConfigVariables,
  useUserConfig,
  parseConfigSection,
  GroupConfig,
  TemplateConfig,
} from './user-config';
import { ElementContext, ElementType, getElementVariables } from './context';
import { useTemplateEngine } from './template-engine';
import { setWindowPosition, setWindowStyles } from './desktop';
import { createDeepSignal, resolved } from './utils';

export async function initWindowAsync() {
  // TODO: Promisify `init`.
}

export function initWindow(callback: (context: ElementContext) => void) {
  const config = useUserConfig();
  const [configVariables] = getConfigVariables();
  const templateEngine = useTemplateEngine();

  // TODO: Remove this.
  const [rootVariables] = createResource(configVariables, configVariables => ({
    env: configVariables,
  }));

  const [windowContext] = createResource(
    () => resolved([config(), rootVariables()]),
    ([config, rootVariables]) => {
      // TODO: Get window to open from launch args.
      const configKey = 'window/bar';
      const windowConfig = (config as UserConfig)[configKey];

      return createElementContext({
        id: configKey,
        config: windowConfig,
        ancestorVariables: [() => rootVariables],
      });
    },
    {
      storage: createDeepSignal,
    },
  );

  const [globalConfig] = createResource(config, config =>
    parseConfigSection(
      templateEngine,
      (config as UserConfig).global,
      GlobalConfigSchema.strip(),
      {},
    ),
  );

  // Dynamically create <style> tag and append it to <head>.
  createEffect(async () => {
    if (globalConfig() && windowContext()) {
      const styleElement = document.createElement('style');
      document.head.appendChild(styleElement);
      styleElement.innerHTML = await buildStyles(
        globalConfig()!,
        windowContext()!,
      );

      return () => document.head.removeChild(styleElement);
    }
  });

  // Set window position based on config values.
  createEffect(async () => {
    if (globalConfig() && windowContext()) {
      const windowConfig = windowContext()!.parsedConfig as WindowConfig;

      await setWindowPosition({
        x: windowConfig.position_x,
        y: windowConfig.position_y,
        width: windowConfig.width,
        height: windowConfig.height,
      });

      await setWindowStyles({
        alwaysOnTop: windowConfig.always_on_top,
        showInTaskbar: windowConfig.show_in_taskbar,
        resizable: windowConfig.resizable,
      });
    }
  });

  let hasRunCallback = false;

  // Invoke callback passed to `init`.
  createEffect(() => {
    if (!hasRunCallback && windowContext()) {
      callback(windowContext()!);
      hasRunCallback = true;
    }
  });
}

export interface InitElementArgs {
  id: string;
  config: WindowConfig | GroupConfig | TemplateConfig;
  ancestorVariables?: Accessor<Record<string, unknown>>[];
}

export function initElement(args: InitElementArgs) {
  const type = getElementType(args.id);

  const childConfigs = getChildConfigs(args.config);
  const childIds = childConfigs.map(([key]) => key);

  const { element, merged } = getElementVariables(args.config);

  return {
    id: args.id,
    rawConfig: args.config,
    parsedConfig,
    variables: merged,
    type,
    childIds,
    initChild: () => {
      const foundConfig = childConfigs.find(([key]) => key === args.id);

      if (!foundConfig) {
        return null;
      }

      const [configKey, childConfig] = foundConfig;

      return initElement({
        config: childConfig,
        id: configKey,
        ancestorVariables: [...(args.ancestorVariables ?? []), element],
      });
    },
  };
}

/**
 * Get child element configs.
 */
function getChildConfigs(config: WindowConfig | GroupConfig | TemplateConfig) {
  return Object.entries(config).filter(
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

function getElementType(id: string) {
  const [type] = id.split('/');

  // TODO: Validate in P1 schema instead.
  if (!Object.values(ElementType).includes(type as ElementType)) {
    throw new Error(`Unrecognized element type '${type}'.`);
  }

  return type as ElementType;
}
