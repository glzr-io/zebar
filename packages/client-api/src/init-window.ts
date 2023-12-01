import { createEffect, createMemo, createResource } from 'solid-js';

import {
  GlobalConfigSchema,
  UserConfig,
  WindowConfig,
  buildStyles,
  getConfigVariables,
  useUserConfig,
  parseConfigSection,
} from './user-config';
import { ElementContext } from './context';
import { useTemplateEngine } from './template-engine';
import { setWindowPosition, setWindowStyles } from './desktop';
import { initElement } from './init-element';

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

  let hasRunCallback = false;

  createEffect(() => {
    if (!hasRunCallback && config() && configVariables()) {
      // TODO: Get window to open from launch args.
      const configKey = 'window/bar';
      const windowContext = initElement({
        id: configKey,
        config: (config() as UserConfig)[configKey],
        ancestorVariables: [() => rootVariables()!],
      });

      const globalConfig = createMemo(() =>
        parseConfigSection(
          templateEngine,
          (config() as UserConfig).global,
          GlobalConfigSchema.strip(),
          {},
        ),
      );

      // Dynamically create <style> tag and append it to <head>.
      createEffect(async () => {
        const styleElement = document.createElement('style');
        document.head.appendChild(styleElement);
        styleElement.innerHTML = await buildStyles(
          globalConfig(),
          windowContext,
        );

        return () => document.head.removeChild(styleElement);
      });

      // Set window position based on config values.
      createEffect(async () => {
        const windowConfig = windowContext.parsedConfig as WindowConfig;

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
      });

      // Invoke callback passed to `init`.
      callback(windowContext);
      hasRunCallback = true;
    }
  });
}
