import { For, createSignal } from 'solid-js';
import { ElementContext, ElementType, init } from 'zebar';

import { TemplateElement } from './template-element.component';
import { GroupElement } from './group-element.component';

export interface WindowElementProps {
  // context: {
  //   store: ElementContext<unknown>;
  // };
}

export function WindowElement(props: WindowElementProps) {
  const [context, setContext] = createSignal<ElementContext | null>(null);

  init(context => setContext(context));

  return (
    <div>
      <For each={context()?.children ?? []}>
        {childContext =>
          childContext.type === ElementType.GROUP ? (
            <GroupElement context={childContext} />
          ) : (
            <TemplateElement context={childContext} />
          )
        }
      </For>
    </div>
  );
}
