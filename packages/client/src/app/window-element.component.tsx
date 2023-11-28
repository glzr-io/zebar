import { For, Show, Suspense, createSignal } from 'solid-js';
import { ElementContext, ElementType, init } from 'zebar';

import { TemplateElement } from './template-element.component';
import { GroupElement } from './group-element.component';

export function WindowElement() {
  const [context, setContext] = createSignal<ElementContext | null>(null);

  init(context => {
    console.log('>>', context);

    setContext(context);
  });

  return (
    <Show when={context()}>
      {context => (
        <div
          id={context().parsedConfig.id}
          class={context().parsedConfig.class_name}
        >
          <For each={context().childIds}>
            {childId => (
              <Suspense fallback={<p>meepleft</p>}>
                {context().getChild(childId)!.type === ElementType.GROUP ? (
                  <GroupElement context={context().getChild(childId)!} />
                ) : (
                  <TemplateElement context={context().getChild(childId)!} />
                )}
              </Suspense>
            )}
          </For>
        </div>
      )}
    </Show>
  );
}
