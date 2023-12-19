import { Match, Show, Switch, createSignal } from 'solid-js';
import { ElementContext, ElementType } from 'zebar';

import { TemplateElement } from './template-element.component';
import { GroupElement } from './group-element.component';

export interface ChildElementProps {
  childId: string;
  parentContext: ElementContext;
}

export function ChildElement(props: ChildElementProps) {
  const [childContext, setChildContext] =
    createSignal<ElementContext | null>(null);

  props.parentContext.initChild(props.childId).then(setChildContext);

  return (
    <Show when={childContext()}>
      {context => (
        <Switch>
          <Match when={context().type === ElementType.GROUP}>
            <GroupElement context={context()} />
          </Match>
          <Match when={context().type === ElementType.TEMPLATE}>
            <TemplateElement context={context()} />
          </Match>
        </Switch>
      )}
    </Show>
  );
}
