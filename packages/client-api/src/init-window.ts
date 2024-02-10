import { createEffect, getOwner, runWithOwner } from 'solid-js';
import { getCurrent as getCurrentWindow } from '@tauri-apps/api/window';

import {
  GlobalConfigSchema,
  type UserConfig,
  type WindowConfig,
  getUserConfig,
  getStyleBuilder,
} from './user-config';
import {
  getOpenWindowArgs,
  setWindowPosition,
  setWindowStyles,
} from './desktop';
import { initElement } from './init-element';
import type { WindowContext } from './element-context.model';
import { ElementType } from './element-type.model';

export function initWindow(callback: (context: WindowContext) => void) {
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
export async function initWindowAsync(): Promise<WindowContext> {
  // TODO: Create new root if owner is null.
  const owner = getOwner()!;
  const config = await getUserConfig();
  const styleBuilder = getStyleBuilder(owner);

  const openArgs =
    window.__ZEBAR_OPEN_ARGS ??
    (await getOpenWindowArgs(await getCurrentWindow().label));

  const windowConfig = (config as UserConfig)[
    `window/${openArgs.windowId}` as const
  ];

  if (!windowConfig) {
    throw new Error(
      `Window '${openArgs.windowId}' doesn't exist in config. Is there a` +
        `property for 'window/${openArgs.windowId}'?`,
    );
  }

  const globalConfig = GlobalConfigSchema.strip().parse(
    (config as UserConfig)?.global ?? {},
  );

  const windowContext = (await initElement({
    id: openArgs.windowId,
    type: ElementType.WINDOW,
    rawConfig: windowConfig,
    globalConfig,
    args: openArgs.args,
    env: openArgs.env,
    ancestorProviders: [],
    owner,
  })) as WindowContext;

  if (windowConfig.global_styles) {
    styleBuilder.setGlobalStyles(windowConfig.global_styles);
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
  await setWindowStyles({
    zOrder: config.z_order,
    showInTaskbar: config.show_in_taskbar,
    resizable: config.resizable,
  });

  await setWindowPosition({
    x: config.position_x,
    y: config.position_y,
    width: config.width,
    height: config.height,
  });
}
