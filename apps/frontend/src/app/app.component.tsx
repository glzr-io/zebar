import { Show } from 'solid-js';
import { configure } from 'nunjucks';

import s from './app.module.scss';
import { Bar } from './bar/bar';
import { useUserConfig } from './shared/user-config';

export function App() {
  const userConfig = useUserConfig();

  // Prevent Nunjucks from escaping HTML.
  configure({ autoescape: false });

  return (
    <Show when={userConfig.barConfig()}>
      {barConfig => (
        <div class={s.app}>
          <Bar id="temp" config={barConfig()} />
        </div>
      )}
    </Show>
  );
}
