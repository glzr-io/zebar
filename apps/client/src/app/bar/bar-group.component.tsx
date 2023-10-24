import { For } from 'solid-js';
import { GroupConfig, BarConfig } from 'zebar';

import { BarComponent } from './bar-component.component';

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
