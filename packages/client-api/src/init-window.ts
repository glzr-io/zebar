import { createEffect, getOwner, runWithOwner } from 'solid-js';

import {
  GlobalConfigSchema,
  UserConfig,
  WindowConfig,
  getConfigVariables,
  getUserConfig,
  getStyleBuilder,
} from './user-config';
import { ElementContext } from './context';
import { setWindowPosition, setWindowStyles } from './desktop';
import { initElement } from './init-element';

export function initWindow(callback: (context: ElementContext) => void) {
  initWindowAsync().then(callback);
}

/**
 * Handles initialization.
 *
 * Steps involved:
 *  * Reading the user config
 *  * Creation of the window context
 *  * Positioning the window
 *  * Building CSS and appending it to `<head>`
 */
export async function initWindowAsync(): Promise<ElementContext> {
  // TODO: Create new root if owner is null.
  const owner = getOwner()!;
  const config = await getUserConfig();
  const configVariables = await getConfigVariables();
  const styleBuilder = getStyleBuilder(owner);

  // TODO: Remove this.
  const rootVariables = { env: configVariables };

  // TODO: Get window to open from launch args.
  const configKey = 'window/bar';
  const windowContext = await initElement({
    id: configKey,
    config: (config as UserConfig)[configKey],
    ancestorVariables: [() => rootVariables],
    owner,
  });

  const globalConfig = GlobalConfigSchema.strip().parse(
    (config as UserConfig).global,
  );

  if (globalConfig.root_styles) {
    styleBuilder.setGlobalStyles(globalConfig.root_styles);
  }

  // Set window position based on config values.
  runWithOwner(owner, () => {
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
  });

  return windowContext;
}

async function redrawWindow(windowConfig: WindowConfig): Promise<void> {
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
