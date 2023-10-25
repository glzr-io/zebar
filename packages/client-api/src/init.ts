import { createEffect } from 'solid-js';

import { setWindowPosition, setWindowStyles } from './desktop';
import { GlobalConfigSchema, buildStyles } from './user-config';
import { parseConfigSection } from './user-config/parse-config-section';

export async function initAsync() {
  const rawConfig = await readConfig();
  const config = createConfigStore(rawConfig);

  const context = createContextStore(config);

  const globalConfig = parseConfigSection(
    rawConfig.global,
    GlobalConfigSchema.strip(),
    {},
  );

  // Dynamically create <style> tag and append it to <head>.
  createEffect(async () => {
    const styleElement = document.createElement('style');
    document.head.appendChild(styleElement);
    styleElement.innerHTML = await buildStyles(globalConfig, context);

    return () => document.head.removeChild(styleElement);
  });

  // Set window position based on config values.
  createEffect(async () => {
    const windowConfig = context.store.parsedConfig;

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

  return {
    config,
    context,
  };
}
