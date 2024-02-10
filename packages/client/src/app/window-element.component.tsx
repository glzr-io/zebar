import { Index, Show, createSignal } from 'solid-js';
import {
  type WindowContext,
  getChildConfigs,
  initWindow,
  toCssSelector,
} from 'zebar';

import { ChildElement } from './child-element.component';

export function WindowElement() {
  const [context, setContext] = createSignal<WindowContext | null>(null);

  initWindow(context => setContext(context));

  return (
    <Show when={context()}>
      {context => (
        <div
          class={context().parsedConfig.class_names.join(' ')}
          id={toCssSelector(context().parsedConfig.id)}
        >
          <Index each={getChildConfigs(context().rawConfig)}>
            {childConfig => (
              <ChildElement
                childId={childConfig().id}
                parentContext={context()}
              />
            )}
          </Index>
        </div>
      )}
    </Show>
  );
}
