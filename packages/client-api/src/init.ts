import { createEffect } from 'solid-js';

import { getConfigVariables, createConfigStore } from './user-config';
import { createContextStore } from './context';
function later(delay: number) {
  return new Promise(function (resolve) {
    setTimeout(resolve, delay);
  });
}

export async function initAsync() {
  // const rawConfig = await readConfig();
  // const config = createConfigStore(rawConfig);
  const config = await createConfigStore();
  const configVariables = await getConfigVariables();

  const context = createContextStore(config, configVariables);

  // const globalConfig = parseConfigSection(
  //   config.global,
  //   GlobalConfigSchema.strip(),
  //   {},
  // );

  // Dynamically create <style> tag and append it to <head>.
  // createEffect(async () => {
  //   const styleElement = document.createElement('style');
  //   document.head.appendChild(styleElement);
  //   // styleElement.innerHTML = await buildStyles(
  //   //   globalConfig,
  //   //   context.store.parsedConfig,
  //   // );

  //   return () => document.head.removeChild(styleElement);
  // });

  // Set window position based on config values.
  // createEffect(async () => {
  //   const windowConfig = context.store.parsedConfig;

  //   // await setWindowPosition({
  //   //   x: windowConfig.position_x,
  //   //   y: windowConfig.position_y,
  //   //   width: windowConfig.width,
  //   //   height: windowConfig.height,
  //   // });

  //   // await setWindowStyles({
  //   //   alwaysOnTop: windowConfig.always_on_top,
  //   //   showInTaskbar: windowConfig.show_in_taskbar,
  //   //   resizable: windowConfig.resizable,
  //   // });
  // });

  // await later(2000);
  return context;
}
