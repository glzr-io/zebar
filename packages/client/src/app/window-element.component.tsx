import { For, Show, createSignal } from 'solid-js';
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
          <For each={context().children ?? []}>
            {childContext =>
              childContext.type === ElementType.GROUP ? (
                <GroupElement context={childContext} />
              ) : (
                <TemplateElement context={childContext} />
              )
            }
          </For>
        </div>
      )}
    </Show>
  );
}
