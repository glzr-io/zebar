import { createEffect, getOwner, runWithOwner } from 'solid-js';
import { getCurrent as getCurrentWindow } from '@tauri-apps/api/window';
import { message as messageDialog } from '@tauri-apps/plugin-dialog';

import {
  GlobalConfigSchema,
  type UserConfig,
  getUserConfig,
  getStyleBuilder,
  parseWithSchema,
} from './user-config';
import {
  getOpenWindowArgs,
  setWindowPosition,
  setWindowStyles,
  type WindowPosition,
  type WindowStyles,
} from './desktop';
import { initElement } from './init-element';
import type { WindowContext } from './element-context.model';
import { ElementType } from './element-type.model';
import { createLogger } from '~/utils';

const logger = createLogger('init-window');

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
  try {
    // TODO: Create new root if owner is null.
    const owner = getOwner()!;
    const config = await getUserConfig();
    const styleBuilder = getStyleBuilder(owner);

    const openArgs =
      window.__ZEBAR_OPEN_ARGS ??
      (await getOpenWindowArgs(getCurrentWindow().label));

    const windowConfig = (config as UserConfig)[
      `window/${openArgs.windowId}` as const
    ];

    if (!windowConfig) {
      throw new Error(
        `Window \`${openArgs.windowId}\` isn\'t defined in the config. ` +
          `Is there a property for \`window/${openArgs.windowId}\`?`,
      );
    }

    const globalConfig = parseWithSchema(
      GlobalConfigSchema.strip(),
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

    // Set global SCSS/CSS styles.
    runWithOwner(owner, () => {
      createEffect(() => {
        if (windowContext.parsedConfig.global_styles) {
          styleBuilder.setGlobalStyles(
            windowContext.parsedConfig.global_styles,
          );
        }
      });
    });

    // Set window position and apply window styles/effects.
    runWithOwner(owner, () => {
      createEffect(async () => {
        // Create `styles` and `position` variables prior to awaiting, such that
        // dependencies are tracked successfully within the effect.
        const styles: Partial<WindowStyles> = {
          zOrder: windowContext.parsedConfig.z_order,
          showInTaskbar: windowContext.parsedConfig.show_in_taskbar,
          resizable: windowContext.parsedConfig.resizable,
        };

        const position: Partial<WindowPosition> = {
          x: windowContext.parsedConfig.position_x,
          y: windowContext.parsedConfig.position_y,
          width: windowContext.parsedConfig.width,
          height: windowContext.parsedConfig.height,
        };

        await setWindowStyles(styles);
        await setWindowPosition(position);
      });
    });

    return windowContext;
  } catch (err) {
    logger.error('Failed to initialize window:', err);

    await messageDialog((err as Error)?.message ?? 'Unknown reason.', {
      title: 'Failed to initialize window!',
      type: 'error',
    });

    // Error during window initialization is unrecoverable, so we close
    // the window.
    await getCurrentWindow().close();

    throw err;
  }
}
