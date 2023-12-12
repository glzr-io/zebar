import { createEffect, createResource, createRoot } from 'solid-js';

import {
  GlobalConfigSchema,
  UserConfig,
  WindowConfig,
  useStyleBuilder,
  getConfigVariables,
  useUserConfig,
} from './user-config';
import { ElementContext } from './context';
import { setWindowPosition, setWindowStyles } from './desktop';
import { initElement } from './init-element';

export async function initWindowAsync() {
  // TODO: Promisify `init`.
}

export function initWindow(callback: (context: ElementContext) => void) {
  const config = useUserConfig();
  const [configVariables] = getConfigVariables();
  const styleBuilder = useStyleBuilder();

  // TODO: Remove this.
  const [rootVariables] = createResource(configVariables, configVariables => ({
    env: configVariables,
  }));

  let hasRunCallback = false;

  createEffect(() => {
    if (!hasRunCallback && config() && configVariables()) {
      // Creating a new root is necessary, otherwise nested effects are disposed
      // on reruns of the parent effect.
      createRoot(() => {
        // TODO: Get window to open from launch args.
        const configKey = 'window/bar';
        const windowContext = initElement({
          id: configKey,
          config: (config() as UserConfig)[configKey],
          ancestorVariables: [() => rootVariables()!],
        });

        const globalConfig = GlobalConfigSchema.strip().parse(
          (config() as UserConfig).global,
        );

        if (globalConfig.root_styles) {
          styleBuilder.setGlobalStyles(globalConfig.root_styles);
        }

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
      });
    }
  });
}
