import { Match, Switch } from 'solid-js';
import { ElementContext, ElementType } from 'zebar';

import { TemplateElement } from './template-element.component';
import { GroupElement } from './group-element.component';

export interface ChildElementProps {
  childId: string;
  context: ElementContext;
}

export function ChildElement(props: any) {
  const child = props.context.initChild(props.childId);

  return (
    <Switch>
      <Match when={child.type === ElementType.GROUP}>
        <GroupElement context={child} />
      </Match>
      <Match when={child.type === ElementType.TEMPLATE}>
        <TemplateElement context={child} />
      </Match>
    </Switch>
  );
}
