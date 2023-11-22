import { For, Show, Suspense } from 'solid-js';
import { ElementContext, ElementType } from 'zebar';

import { TemplateElement } from './template-element.component';

export interface GroupElementProps {
  context: ElementContext;
}

export function GroupElement(props: GroupElementProps) {
  console.log('props', props);

  const config = props.context.parsedConfig;

  if (config.id === 'left') {
    console.log('>>', props.context.getChild('template/clock'));
  }

  return (
    <div id={config.id} class={config.class_name}>
      <Show when={config.id === 'left'}>
        <Suspense fallback={<p>meepclock</p>}>
          <TemplateElement context={props.context.getChild('template/clock')} />
        </Suspense>
      </Show>
      <Show when={config.id === 'right'}>
        <Suspense fallback={<p>meepcpu</p>}>
          <TemplateElement context={props.context.getChild('template/cpu')} />
        </Suspense>
      </Show>
    </div>
  );
}
