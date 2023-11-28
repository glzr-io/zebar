import { For, Suspense } from 'solid-js';
import { ElementContext, ElementType } from 'zebar';

import { TemplateElement } from './template-element.component';

export interface GroupElementProps {
  context: ElementContext;
}

export function GroupElement(props: GroupElementProps) {
  const config = props.context.parsedConfig;

  return (
    <div id={config.id} class={config.class_name}>
      <For each={props.context.childIds}>
        {childId => (
          <Suspense fallback={<p>meepleft</p>}>
            {props.context.getChild(childId)!.type === ElementType.GROUP ? (
              <GroupElement context={props.context.getChild(childId)!} />
            ) : (
              <TemplateElement context={props.context.getChild(childId)!} />
            )}
          </Suspense>
        )}
      </For>
    </div>
  );
}
