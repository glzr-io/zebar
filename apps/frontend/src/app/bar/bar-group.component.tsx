import { For } from 'solid-js';

import { BarComponent } from './bar-component.component';
import { ProviderNode } from '~/shared/providers';
import { GroupConfig } from '~/shared/user-config';

export interface BarGroupProps {
  config: GroupConfig;
  provider: ProviderNode;
}

export function BarGroup(props: BarGroupProps) {
  return (
    <div id={props.config.id} class={props.config.class_name}>
      <For each={props.config.components}>
        {componentConfig => (
          <BarComponent config={componentConfig} provider={props.provider} />
        )}
      </For>
    </div>
  );
}
