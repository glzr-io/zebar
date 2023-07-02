import { Show } from 'solid-js';
import { compileString } from 'sass';

import s from './app.module.scss';
import { useConfig } from './shared/use-config.hook';

export function App() {
  const config = useConfig();

  console.log(
    compileString(`
      .box {
        width: 10px + 15px;
      }
    `),
  );

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
