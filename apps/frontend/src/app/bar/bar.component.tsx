import { For } from 'solid-js';
import { BarConfig, getGroupConfigs } from 'zebar';

import { BarGroup } from './bar-group.component';

export interface BarProps {
  config: BarConfig;
}

export function Bar(props: BarProps) {
  return (
    <div id={props.config.id} class={props.config.class_name}>
      <For each={getGroupConfigs(props.config)}>
        {groupConfig => (
          <BarGroup config={groupConfig} parentConfig={props.config} />
        )}
      </For>
    </div>
  );
}
