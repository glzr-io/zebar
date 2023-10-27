import { For, Index, Show, createEffect, createSignal } from 'solid-js';
import { ElementContext, ElementType, initAsync } from 'zebar';

import { TemplateElement } from './template-element.component';
import { GroupElement } from './group-element.component';

export interface WindowElementProps {
  // context: any;
}

export function WindowElement(props: WindowElementProps) {
  const [context, setContext] = createSignal<any>(null);

  // const config = props.context.store.parsedConfig;
  // createEffect(() => console.log('config changed', config, props.context));

  createEffect(async () => {
    const a = await initAsync();
    setContext(a);
    console.log('context', context(), context().store);
  });

  return (
    // <div id={config.id} class={config.class_name}>
    <Show when={context() !== null}>
      <div>
        <Index each={context().store.children}>
          {childContext =>
            childContext().type === ElementType.GROUP ? (
              <GroupElement context={childContext()} />
            ) : (
              <TemplateElement context={childContext()} />
            )
          }
        </Index>
      </div>
    </Show>
  );
}
