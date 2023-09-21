import { Show, createEffect, on } from 'solid-js';
import { configure } from 'nunjucks';

import { Bar } from './bar/bar';
import { useStyleBuilder, useUserConfig } from './shared/user-config';
import { useCurrentWindow } from './shared/desktop';
import { resolved } from './shared/utils';

export function App() {
  const userConfig = useUserConfig();
  const styleBuilder = useStyleBuilder();
  const currentWindow = useCurrentWindow();

  // Prevent Nunjucks from escaping HTML.
  configure({ autoescape: false });

  // Set bar position based on config values.
  createEffect(
    on(
      () => userConfig.barConfig(),
      async barConfig => {
        if (barConfig) {
          await currentWindow.setPosition({
            x: barConfig.position_x,
            y: barConfig.position_y,
            width: barConfig.width,
            height: barConfig.height,
          });

          await currentWindow.setStyles({
            alwaysOnTop: barConfig.always_on_top,
            showInTaskbar: barConfig.show_in_taskbar,
            resizable: barConfig.resizable,
          });
        }
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
      { defer: true },
    ),
  );

  return (
    <Show
      when={resolved([userConfig.barConfig(), styleBuilder.builtCss()])}
      keyed
    >
      {/* Mount bar when built CSS + bar config is ready. */}
      {([barConfig]) => <Bar config={barConfig} />}
    </Show>
  );
}
