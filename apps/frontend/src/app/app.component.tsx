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
