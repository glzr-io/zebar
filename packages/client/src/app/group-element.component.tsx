import { For } from 'solid-js';
import { ElementContext, ElementType } from 'zebar';

import { TemplateElement } from './template-element.component';

export interface GroupElementProps {
  context: ElementContext;
}

export function GroupElement(props: GroupElementProps) {
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
