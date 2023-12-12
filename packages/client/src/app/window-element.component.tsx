import { Index, Show, Suspense, createSignal } from 'solid-js';
import { ElementContext, initWindow, toCssSelector } from 'zebar';

import { ChildElement } from './child-element.component';

export function WindowElement() {
  const [context, setContext] = createSignal<ElementContext | null>(null);

  initWindow(context => setContext(context));

  return (
    <Show when={context()}>
      {context => (
        <div
          id={toCssSelector(context().parsedConfig.id)}
          class={context().parsedConfig.class_name}
        >
          <Index each={context().childIds}>
            {childId => (
              <Suspense>
                <ChildElement childId={childId()} context={context()} />
              </Suspense>
            )}
          </Index>
        </div>
      )}
    </Show>
  );
}
