import { Show, createMemo } from 'solid-js';

import s from './app.module.scss';
import { Bar } from './bar/bar';
import { useConfig } from './shared/use-config.hook';

export function App() {
  const config = useConfig();

  const barConfig = createMemo(() => config()?.['bar/main']);

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
