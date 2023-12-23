import { Index, Show, createSignal } from 'solid-js';
import {
  WindowContext,
  getChildIds,
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
          id={toCssSelector(context().parsedConfig.id)}
          class={context().parsedConfig.class_name}
        >
          <Index each={getChildIds(context().rawConfig)}>
            {childId => (
              <ChildElement
                childId={childId()}
                parentContext={context()}
              />
            )}
          </Index>
        </div>
      )}
    </Show>
  );
}
