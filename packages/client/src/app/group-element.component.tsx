import { Index } from 'solid-js';
import {
  type ElementContext,
  getChildConfigs,
  toCssSelector,
} from 'zebar';

import { ChildElement } from './child-element.component';

export interface GroupElementProps {
  context: ElementContext;
}

export function GroupElement(props: GroupElementProps) {
  const config = props.context.parsedConfig;
  const rawConfig = props.context.rawConfig;

  return (
    <div
      class={config.class_names.join(' ')}
      id={toCssSelector(config.id)}
    >
      <Index each={getChildConfigs(rawConfig)}>
        {childConfig => (
          <ChildElement
            childId={childConfig().id}
            parentContext={props.context}
          />
        )}
      </Index>
    </div>
  );
}
