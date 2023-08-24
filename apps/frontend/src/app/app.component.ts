import { createEffect, on } from 'solid-js';
import { configure } from 'nunjucks';

import { Bar } from './bar/bar';
import { useStyleBuilder, useUserConfig } from './shared/user-config';
import { useCurrentWindow } from './shared/desktop';
import { mount } from './shared/utils';

export function App(): Element {
  const userConfig = useUserConfig();
  const styleBuilder = useStyleBuilder();
  const currentWindow = useCurrentWindow();

  const element = document.createElement('div');

  // Prevent Nunjucks from escaping HTML.
  configure({ autoescape: false });

  // Mount bar when built CSS + user config is ready.
  createEffect(
    on(
      () => [styleBuilder.builtCss(), userConfig.barConfig()] as const,
      ([builtCss, barConfig]) => {
        if (builtCss && barConfig) {
          mount(element, Bar(barConfig));
        }
      },
    ),
  );

  // Set bar position based on config values.
  createEffect(
    on(
      () => userConfig.generalConfig(),
      async generalConfig => {
        if (generalConfig) {
          await currentWindow.setPosition({
            x: generalConfig.position_x,
            y: generalConfig.position_y,
            width: generalConfig.width,
            height: generalConfig.height,
          });

          await currentWindow.setStyles({
            alwaysOnTop: generalConfig.alwaysOnTop,
            showInTaskbar: generalConfig.showInTaskbar,
            resizable: generalConfig.resizable,
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

  return element;
}
