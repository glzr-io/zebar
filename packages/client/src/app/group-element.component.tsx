import { Index } from 'solid-js';
import { ElementContext, toCssSelector } from 'zebar';

import { ChildElement } from './child-element.component';

export interface GroupElementProps {
  context: ElementContext;
}

export function GroupElement(props: GroupElementProps) {
  const config = props.context.parsedConfig;

  return (
    <div id={toCssSelector(config.id)} class={config.class_name}>
      <Index each={props.context.childIds}>
        {childId => (
          <ChildElement childId={childId()} parentContext={props.context} />
        )}
      </Index>
    </div>
  );
}
