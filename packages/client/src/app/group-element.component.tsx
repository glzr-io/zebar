import { Index, createMemo } from 'solid-js';
import {
  ElementContext,
  GroupConfig,
  getChildConfigs,
  toCssSelector,
} from 'zebar';

import { ChildElement } from './child-element.component';

export interface GroupElementProps {
  context: ElementContext;
}

export function GroupElement(props: GroupElementProps) {
  const config = props.context.parsedConfig;

  const childIds = createMemo(() =>
    getChildConfigs(props.context.rawConfig as GroupConfig).map(
      ([key]) => key,
    ),
  );

  return (
    <div id={toCssSelector(config.id)} class={config.class_name}>
      <Index each={childIds()}>
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
