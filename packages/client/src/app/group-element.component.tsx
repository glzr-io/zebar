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
            {(() => {
              const child = props.context.initChild(childId)!;

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
  );
}
