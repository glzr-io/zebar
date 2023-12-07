import { For, Show, Suspense, createSignal } from 'solid-js';
import { ElementContext, ElementType, initWindow } from 'zebar';

import { TemplateElement } from './template-element.component';
import { GroupElement } from './group-element.component';

export function WindowElement() {
  const [context, setContext] = createSignal<ElementContext | null>(null);

  initWindow((context: any) => {
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
                {(() => {
                  const child = context().initChild(childId)!;

                  return child.type === ElementType.GROUP ? (
                    <GroupElement context={child} />
                  ) : (
                    <TemplateElement context={child} />
                  );
                })()}
              </Suspense>
            )}
          </For>
        </div>
      )}
    </Show>
  );
}
