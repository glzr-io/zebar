import { Show } from 'solid-js';

import s from './app.module.scss';
import { useConfig } from './shared/use-config.hook';

export function App() {
  const config = useConfig();

  return (
    <Show when={config()}>
      {config => (
        <div class={s.app}>
          <p>Hello</p>
          <p>{JSON.stringify(config())}</p>
        </div>
      )}
    </Show>
  );
}
