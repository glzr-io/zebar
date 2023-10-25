import { createEffect } from 'solid-js';

import { setWindowPosition, setWindowStyles } from './desktop';
import {
  GlobalConfigSchema,
  UserConfig,
  buildStyles,
  getConfigVariables,
  readUserConfig,
} from './user-config';
import { parseConfigSection } from './user-config/parse-config-section';
import { createContextStore } from './context';

export async function initAsync() {
  // const rawConfig = await readConfig();
  // const config = createConfigStore(rawConfig);
  const config = (await readUserConfig()) as UserConfig;
  const configVariables = await getConfigVariables();

  const context = createContextStore(config, configVariables);

  const globalConfig = parseConfigSection(
    config.global,
    GlobalConfigSchema.strip(),
    {},
  );

  // Dynamically create <style> tag and append it to <head>.
  createEffect(async () => {
    const styleElement = document.createElement('style');
    document.head.appendChild(styleElement);
    // styleElement.innerHTML = await buildStyles(
    //   globalConfig,
    //   context.store.parsedConfig,
    // );

    return () => document.head.removeChild(styleElement);
  });

  // Set window position based on config values.
  createEffect(async () => {
    const windowConfig = context.store.parsedConfig;

    // await setWindowPosition({
    //   x: windowConfig.position_x,
    //   y: windowConfig.position_y,
    //   width: windowConfig.width,
    //   height: windowConfig.height,
    // });

    // await setWindowStyles({
    //   alwaysOnTop: windowConfig.always_on_top,
    //   showInTaskbar: windowConfig.show_in_taskbar,
    //   resizable: windowConfig.resizable,
    // });
  });

  return context;
}
