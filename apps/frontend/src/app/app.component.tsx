import { Show, createMemo } from 'solid-js';
import { configure } from 'nunjucks';

import s from './app.module.scss';
import { Bar } from './bar/bar';
import { useUserConfig } from './shared/user-config';

export function App() {
  const userConfig = useUserConfig();

  const barConfig = createMemo(() => userConfig()?.['bar/main']);

  // Prevent Nunjucks from escaping HTML.
  configure({ autoescape: false });

  return (
    <Show when={barConfig()}>
      {config => (
        <div class={s.app}>
          <Bar id="temp" config={config()} />
        </div>
      )}
    </Show>
  );
}
