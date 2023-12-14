import { createEffect, getOwner, runWithOwner } from 'solid-js';

import {
  GlobalConfigSchema,
  UserConfig,
  WindowConfig,
  getConfigVariables,
  getUserConfig,
  getStyleBuilder,
} from './user-config';
import { setWindowPosition, setWindowStyles } from './desktop';
import { initElement } from './init-element';
import { ElementContext } from './element-context.model';

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
    createEffect(() =>
      redrawWindow(windowContext.parsedConfig as WindowConfig),
    );
  });

  return windowContext;
}

async function redrawWindow(config: WindowConfig): Promise<void> {
  await setWindowPosition({
    x: config.position_x,
    y: config.position_y,
    width: config.width,
    height: config.height,
  });

  await setWindowStyles({
    alwaysOnTop: config.always_on_top,
    showInTaskbar: config.show_in_taskbar,
    resizable: config.resizable,
  });
}
