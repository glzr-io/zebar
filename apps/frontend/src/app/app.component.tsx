import { Show, createMemo } from 'solid-js';

import s from './app.module.scss';
import { useConfig } from './shared/use-config.hook';
import { ComponentGroup } from './component-group/component-group';

export function App() {
  const config = useConfig();

  const barConfig = createMemo(() => config()?.['bar/main']);

  return (
    <Show when={barConfig()}>
      {config => (
        <div class={s.app}>
          <ComponentGroup id="temp" config={config().components_left} />
          <ComponentGroup id="temp" config={config().components_middle} />
          <ComponentGroup id="temp" config={config().components_right} />
        </div>
      )}
    </Show>
  );
}
