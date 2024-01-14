import { Index } from 'solid-js';
import { type ElementContext, getChildIds, toCssSelector } from 'zebar';

import { ChildElement } from './child-element.component';

export interface GroupElementProps {
  context: ElementContext;
}

export function GroupElement(props: GroupElementProps) {
  const config = props.context.parsedConfig;
  const rawConfig = props.context.rawConfig;

  return (
    <div
      id={toCssSelector(config.id)}
      class={config.class_names.join(' ')}
    >
      <Index each={getChildIds(rawConfig)}>
        {childId => (
          <ChildElement
            childId={childId()}
            parentContext={props.context}
          />
        )}
      </Index>
    </div>
  );
}
