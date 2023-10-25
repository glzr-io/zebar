import { For } from 'solid-js';
import { ElementContext, ElementType } from 'zebar';

import { TemplateElement } from './template-element.component';
import { GroupElement } from './group-element.component';

export interface WindowElementProps {
  context: ElementContext;
}

export function WindowElement(props: WindowElementProps) {
  const config = props.context.parsedConfig;

  return (
    <div id={config.id} class={config.class_name}>
      <For each={props.context.children}>
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
