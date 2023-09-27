import { For } from 'solid-js';

import { BarComponent } from './bar-component.component';
import { BarConfig, GroupConfig } from '~/shared/user-config';

export interface BarGroupProps {
  config: GroupConfig;
  parentConfig: BarConfig;
}

export function BarGroup(props: BarGroupProps) {
  return (
    <div id={props.config.id} class={props.config.class_name}>
      <For each={props.config.components}>
        {componentConfig => (
          <BarComponent config={componentConfig} parentConfig={props.config} />
        )}
      </For>
    </div>
  );
}
