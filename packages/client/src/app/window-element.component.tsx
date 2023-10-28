import {
  For,
  Index,
  createEffect,
  createMemo,
  createResource,
  createSignal,
} from 'solid-js';
import { createStore } from 'solid-js/store';
import { ElementContext, ElementType, init, initAsync } from 'zebar';

import { TemplateElement } from './template-element.component';
import { GroupElement } from './group-element.component';

export interface WindowElementProps {
  // context: {
  //   store: ElementContext<unknown>;
  // };
}

export function WindowElement(props: WindowElementProps) {
  // const [context, setContext] = createStore(props.context.store);
  // const [context] = createResource(() => initAsync());
  // const [context] = createResource(() => initAsync().then(res => res.store));
  const [context, setContext] = createSignal<ElementContext<unknown> | null>(
    null,
  );
  // const config = createMemo(() => context().parsedConfig);

  init(context => {
    console.log('context', context);
    setContext(context);
  });
  // initAsync().then(res => setContext(res));

  createEffect(() =>
    // console.log('>>>', context(), context().children, context().children?.[0]),
    console.log(
      '>>>',
      context(),
      context()?.children,
      // context()?.store.children?.[0],
    ),
  );

  return (
    // <div id={config.id} class={config.class_name}>
    <div>
      <For each={context()?.children ?? []}>
        {childContext => {
          console.log('childContext', childContext);

          return childContext.type === ElementType.GROUP ? (
            <GroupElement context={childContext} />
          ) : (
            <TemplateElement context={childContext} />
          );
        }}
      </For>
    </div>
  );
}
