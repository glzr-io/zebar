import {
  PhysicalPosition,
  PhysicalSize,
  getCurrent as getCurrentWindow,
} from '@tauri-apps/api/window';
import { Show, createEffect, on } from 'solid-js';
import { configure } from 'nunjucks';

import { Bar } from './bar/bar';
import { useStyleBuilder, useUserConfig } from './shared/user-config';
import { resolved } from './shared/utils';

export function App() {
  const userConfig = useUserConfig();
  const styleBuilder = useStyleBuilder();

  // Prevent Nunjucks from escaping HTML.
  configure({ autoescape: false });

  // Set bar position based on config values.
  createEffect(
    on(
      () => userConfig.generalConfig(),
      async generalConfig => {
        // TODO: Default to x = 0, y = 0, width = 100%, height = 50px.
        const x = eval(generalConfig?.position_x!);
        const y = eval(generalConfig?.position_y!);
        const width = eval(generalConfig?.width!);
        const height = eval(generalConfig?.height!);

        getCurrentWindow().setPosition(new PhysicalPosition(x, y));
        getCurrentWindow().setSize(new PhysicalSize(width, height));
      },
      { defer: true },
    ),
  );

  // Dynamically create <style> tag and append it to <head>.
  createEffect(
    on(
      () => styleBuilder.builtCss(),
      builtCss => {
        if (builtCss) {
          const styleElement = document.createElement('style');
          document.head.appendChild(styleElement);
          styleElement.innerHTML = builtCss;

          return () => document.head.removeChild(styleElement);
        }
      },
    ),
  );

  return (
    <Show
      when={resolved([userConfig.barConfig(), styleBuilder.builtCss()])}
      keyed
    >
      {([barConfig]) => <Bar config={barConfig} />}
    </Show>
  );
}
