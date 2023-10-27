import { For, Index, createEffect, createMemo } from 'solid-js';
import { ElementContext, ElementType } from 'zebar';

import { TemplateElement } from './template-element.component';
import { GroupElement } from './group-element.component';

export interface WindowElementProps {
  context: {
    store: ElementContext<unknown>;
  };
}

export function WindowElement(props: WindowElementProps) {
  const context = createMemo(() => props.context.store);
  const config = createMemo(() => context().parsedConfig);

  createEffect(() =>
    console.log('>>>', context(), context().children, context().children?.[0]),
  );

  return (
    <div id={config().id} class={config().class_name}>
      <Index each={context().children}>
        {childContext =>
          childContext().type === ElementType.GROUP ? (
            <GroupElement context={childContext()} />
          ) : (
            <TemplateElement context={childContext()} />
          )
        }
      </Index>
    </div>
  );
}
