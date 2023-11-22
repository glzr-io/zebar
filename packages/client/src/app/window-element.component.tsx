import { For, Show, Suspense, createSignal } from 'solid-js';
import { ElementContext, ElementType, init } from 'zebar';

import { TemplateElement } from './template-element.component';
import { GroupElement } from './group-element.component';

export function WindowElement() {
  const [context, setContext] = createSignal<ElementContext | null>(null);

  init(context => setContext(context));

  return (
    <Show when={context()}>
      {context => (
        <div
          id={context().parsedConfig.id}
          class={context().parsedConfig.class_name}
        >
          <Suspense fallback={<p>meepleft</p>}>
            <GroupElement context={context().getChild('group/left')} />
          </Suspense>

          <Suspense fallback={<p>meep</p>}>
            <GroupElement context={context().getChild('group/right')} />
          </Suspense>
        </div>
      )}
    </Show>
  );
}
