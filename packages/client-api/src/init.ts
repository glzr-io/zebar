import { createEffect, createMemo, createResource } from 'solid-js';

import {
  GlobalConfigSchema,
  UserConfig,
  WindowConfig,
  buildStyles,
  getConfigVariables,
  getUserConfig,
  parseConfigSection,
} from './user-config';
import { ElementContext, createElementContext } from './context';
import { useTemplateEngine } from './template-engine';
import { setWindowPosition, setWindowStyles } from './desktop';

export async function initAsync() {
  // TODO: Promisify `init`.
}

export function init(callback: (context: ElementContext) => void) {
  const [config] = getUserConfig();
  const [configVariables] = getConfigVariables();
  const templateEngine = useTemplateEngine();

  // TODO: Get window to open from launch args.
  const configKey = 'window/bar';
  const windowConfig = (config() as UserConfig)[configKey];

  // TODO: Remove this.
  const rootVariables = createMemo(() => ({ env: configVariables() }));

  const context = createElementContext({
    id: configKey,
    config: windowConfig,
    ancestorVariables: [rootVariables],
  });

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
    if (globalConfig() && context.store.hasInitialized) {
      const styleElement = document.createElement('style');
      document.head.appendChild(styleElement);
      styleElement.innerHTML = await buildStyles(
        globalConfig()!,
        context.store.value!,
      );

      return () => document.head.removeChild(styleElement);
    }
  });

  // Set window position based on config values.
  createEffect(async () => {
    if (globalConfig() && context.store.hasInitialized) {
      const windowConfig = context.store.value!.parsedConfig as WindowConfig;

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

  // Invoke callback passed to `init`.
  createEffect(() => {
    if (context.store.hasInitialized) {
      callback(context.store.value!);
    }
  });
}
